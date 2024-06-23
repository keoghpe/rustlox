23-06-2024

- [x] Find out where I am in this book
    - looks like I'm around page 161
- [x] Implement returnStatement function
- [ ] Implement returning from calls
    - [x] How can we implement the equivalent of raising a custom exception & rescuing in the call function
        The equivalent of the return object return needs to refer to the value that's being returned
        We can have a return struct that refers to the value
    - [ ] What are all the places that can be between call and return?
    - [ ] Implement return struct
    - Pause on branch: `implement_return_statement` - maybe I need to implement proper runtime errors before I implement return

- [ ] Replace all panic!s with proper handled errors
