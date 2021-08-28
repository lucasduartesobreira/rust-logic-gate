# Logic Ports in Rust using somewhat System F notation

That's an implementation test of logic ports in Rust using the System F-like notation of booleans.

## What is happening under the hood?

### Boolean definition
Define a Boolean as something that takes the other two Booleans as arguments and returns a Boolean.

This definition enables the creation of two types, True and False, which are Booleans, the True and False functions returns, respectively, first and the second argument.

Because of the way Rust restrictions and my knowledge about it, those two types need to contain the actual bool value and a function that returns the value. That's why exists a get_value function and the initialization default function

### Logic Ports
The logic ports are represented by: \
**AND** - $\lambda$$x^{ Boolean }$$\lambda$y.x Boolean y ***F*** \
**OR** - $\lambda$$x^{ Boolean }$$\lambda$y.x Boolean ***T*** y \
**NOT** - $\lambda$$x^{ Boolean }$.x Boolean ***F*** ***T*** 

That can be readen by: \
**AND** - takes a *x* thats a Boolean and a *y*, consider *y* and ***F*** as booleans and apply *y* and ***F*** into x \
**OR** - takes a *x* thats a Boolean and a *y*, consider *y* and ***T*** as booleans and apply *y* and ***F*** into x \
**NOT** - takes a *x* thats a boolean and apply to it ***F*** and ***T*** as booleans.

Verbose explanation of the logic ports: \
**AND** - Case x is True, it will return the first argument that happens to be y, so if y is True, the port returns true. Otherwise returns false. \
**OR** - In case x is True, it will return the first argument that happens to be True. Case x is False, it will return the second argument that happens to be y, so if y is True, then the port returns true. Otherwise returns false. \
**NOT** - In case x is True, it will return the first argument that happens to be False. In case x is False, it will return the second argument that happens to be True.
