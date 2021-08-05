--TODO--
Spin up on server
Fix failing test cases

--Installation--
A functional Windows C++ Dev Tools installation is required
Rust installation
Execute with cargo run

--Execute on port 80--
ROCKET_PORT 80 cargo run


--Expected behavior--
Addition of a dollar sign in the first position [0] will convert to dollar mode, in which all amounts are communicated as x dollars, and y cents
Dollar amounts are restricted to full cents
Non-Dollar amounts are not restricted in decimal values
Invalid characters are cleaned from the input rather than triggering a 400 response