use std::collections::{HashMap, VecDeque};

struct InternalCell<'a, T> {
    value: T,
    compute_func: Option<Box<dyn Fn(&[T]) -> T>>,
    deps: Vec<CellID>,
    callbacks: HashMap<CallbackID, Box<dyn FnMut(T) -> () + 'a>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    id_pool: Box<dyn Iterator<Item = usize>>,
    cells: HashMap<CellID, InternalCell<'a, T>>,
    deps: HashMap<CellID, Vec<CellID>>,
    dirty: VecDeque<CellID>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            id_pool: Box::new(0..),
            cells: HashMap::new(),
            deps: HashMap::new(),
            dirty: VecDeque::new(),
        }
    }

    fn new_id(&mut self) -> usize {
        self.id_pool.next().expect("Internal ID Pool exhausted")
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let id = InputCellID(self.new_id());
        let cell = InternalCell {
            value: initial,
            compute_func: None,
            deps: Vec::new(),
            callbacks: HashMap::new(),
        };
        self.cells.insert(CellID::Input(id), cell);
        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F>(
        &mut self,
        deps: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID>
    where
        F: 'static + Fn(&[T]) -> T,
    {
        if let Some(&missing) = deps.iter().find(|dep| !self.cells.contains_key(&dep)) {
            return Err(missing);
        }

        let id = ComputeCellID(self.new_id());

        let mut dep_values = Vec::new();
        for &dep in deps {
            self.deps.entry(dep).or_default().push(CellID::Compute(id));
            dep_values.push(self.value(dep).unwrap());
        }

        let cell = InternalCell::<T> {
            value: compute_func(&dep_values),
            compute_func: Some(Box::new(compute_func)),
            deps: deps.to_vec(),
            callbacks: HashMap::new(),
        };
        self.cells.insert(CellID::Compute(id), cell);
        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        self.cells.get(&id).map(|v| v.value)
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        let id = CellID::Input(id);
        if let Some(cell) = self.cells.get_mut(&id) {
            cell.value = new_value;
            self.mark_dirty(&id);
            self.react();
            true
        } else {
            false
        }
    }

    fn mark_dirty(&mut self, id: &CellID) {
        if let Some(deps) = self.deps.get(id) {
            self.dirty.extend(deps);
        }
    }

    fn react(&mut self) {
        while let Some(id) = self.dirty.pop_front() {
            let cell = self.cells.get(&id).unwrap();
            let deps = cell.deps.clone();

            if deps.iter().any(|dep| self.dirty.contains(dep)) {
                self.dirty.push_back(id);
                continue;
            }

            let dep_values: Vec<_> = deps.iter().map(|&dep| self.value(dep).unwrap()).collect();

            let value = cell.compute_func.as_ref().unwrap()(&dep_values);
            if cell.value != value {
                let cell = self.cells.get_mut(&id).unwrap();
                cell.value = value;
                for callback in cell.callbacks.values_mut() {
                    callback(value);
                }
                self.mark_dirty(&id);
            }
        }
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F>(&mut self, id: ComputeCellID, callback: F) -> Option<CallbackID>
    where
        F: 'a + FnMut(T) -> (),
    {
        let callback_id = CallbackID(self.new_id());
        if let Some(cell) = self.cells.get_mut(&CellID::Compute(id)) {
            cell.callbacks.insert(callback_id, Box::new(callback));
            Some(callback_id)
        } else {
            None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(cell) = self.cells.get_mut(&CellID::Compute(cell)) {
            cell.callbacks
                .remove(&callback)
                .map(|_| ())
                .ok_or(RemoveCallbackError::NonexistentCallback)
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }
}
