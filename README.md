Basic Structure of the crate is creating Market module + Account module + Strategy module, and Market&Account need to receive websocket data and call Strategy's callback methods.

All variables about trading strategy should be holding in Strategy module, and modified by Market&Account event asynchronously.

It is originally designed as running both modules in a single thread but spawn methods require static lifetime and a Rc<RefCell<Strategy>> module cannot be moved to both modules simultaneously.
The two modules are joined at main thread and because of the two threads may conflict when reaching for mut_borrow of RefCell<Strategy>,
so a async mutex is used to lock the Strategy module and await until unlock whenever conflicting.
