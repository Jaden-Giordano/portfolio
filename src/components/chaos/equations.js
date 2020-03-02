import { Point } from '../../utils/math';

export default [
    ({x, y}, t) => new Point(
        (-Math.pow(x, 2) + (x * t) + y),
        (Math.pow(x, 2) - Math.pow(y, 2) - Math.pow(t, 2) - (x * y) + (y * t) - x + y),
    ),
    ({x, y}, t) => new Point(
        -Math.pow(t, 2) + (x * y) + t,
        -(x * y) + (x * t) + y + t,
    ),
];
