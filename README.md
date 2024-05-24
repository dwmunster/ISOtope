# ISOtope

The **I**terative **SO**lver for 2D Sketches. This project is a dependency free gradient descent based solver. It works by minimizing the energy of virtual springs that represent the constraints.

- Keeps it simple and stupid. Most PCs are more than powerful enough to solve small sketches in real time. We don't need to overcomplicate things.
- Each Sketch is a list of primitives and constraints.
- Supported primitives
    - [x] Point
    - [x] Line
    - [x] Circle
    - [x] Arc
- Each constraint is a virtual spring and has a function to calculate its energy (or call it loss function) and the gradient. Current constraints are:
    - [ ] Distance
    - [ ] Angle
    - [x] Horizontal
    - [x] Vertical
    - [x] Coincident
    - [ ] Tangent
    - [ ] Concentric
    - [ ] Equal
    - [ ] Parallel
    - [ ] Perpendicular
    - [ ] Fix

## Algorithm

The algorithm is quite simple. First, we create a Sketch that constists of primitives. Primitives can also reference each other. Take for example a line that is defined by two points. If one of the points moves, the line should also move. This is done by creating a reference to the point in the line. The line will then always calculate its position based on the points.

Consider for example a simple sketch consisting of three points $p_a$, $p_b$ and $p_c$, and two lines $l_1$ and $l_2$. The vector $q$ would look like this:

$$
p_a = [x_1, y_1]
$$

$$
p_b = [x_2, y_2]
$$

$$
p_c = [x_3, y_3]
$$

$$
l_1 = \overline{p_a p_b}
$$

$$
l_2 = \overline{p_b p_c}
$$

Now we take all these parameters and put them into a big vector $q$. This vector is the current state of our sketch. 

$$
q = [x_1, y_1, x_2, y_2, x_3, y_3]
$$

This is great, now all we have to do is to find the value for $q$ that satisfies all the constraints. We do this by minimizing the energy of the constraints. The energy of a constraint is a function that takes the current state of the sketch and returns a scalar value. This value is the energy of the constraint. The gradient of the energy is a vector that points in the direction of the steepest ascent. We can use this gradient to update the state of the sketch. 

If you want to have a visual interpretation, think of physical springs. [The energy of a spring is given by](https://en.wikipedia.org/wiki/Hooke%27s_law#Spring_energy)

$$
E = 0.5 * k * x^2
$$

We model all our constraints the same way. Take for example a coincident constraint.

$$
L(x_1, y_1, x_2, y_2) = 0.5 * {(x_1 - x_2)^2 + (y_1 - y_2)^2}
$$

Or a horizontal constraint

$$
L(x_1, y_1, x_2, y_2) = 0.5 * (y_1 - y_2)^2
$$

Some constraints are more complex, like a distance constraint, but in the end all of them are a function that map the current state of the sketch to a scalar value which we want to minimize.

Now all we have to do is a simple gradient descent

$$
q_{t+1} = q_t - \alpha \sum_i \nabla L_i(q_t)
$$

Where $\alpha$ is the step size (typicall 0.001) and $\nabla L_i(q_t)$ is the gradient of the energy of the constraints. If we do this often enough, e.g. 100000 times, we will find a state of the sketch that satisfies all the constraints as much as possible. It ends up in the same phyical state as a spring system would. Even though 100000 times sounds a like a lot, it is not for a modern PC. It just takes a couple of milliseconds.

In case of conflicting constraints, we can also figure out which constraints are the ones that actually cause the conflict. Constraints that are satisfied will have a energy/loss of 0.

$$
L_i(q) = 0
$$

constraints that are in conflict will have a non-zero energy/loss.

$$
L_i(q) > 0
$$

and can be highlighted to the user.


## Todos

- [x] When adding a constraint, check that all primitives are already in the sketch

## Usage
