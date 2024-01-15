Basic Structure of the crate is creating Market module + Account module + Strategy module, and Market&Account need to receive websocket data and call Strategy's callback methods.

All variables about trading strategy should be holding in Strategy module, and modified by Market&Account event asynchronously.

It is originally designed as running both modules in a single thread but spawn methods require static lifetime and a Rc<RefCell<Strategy>> module cannot be moved to both modules simultaneously.

To meet the lifetime requirement, the two modules are joined at main thread. However under multi-threads asynchronous structure, threads may conflict when reaching for mutable borrow of RefCell<Strategy>, 
so RefCell is abandoned and a async mutex is used to lock the Strategy module and await until unlock whenever conflict arising.
