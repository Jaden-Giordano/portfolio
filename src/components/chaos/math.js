import { Point } from '../../utils/math';

export class ChaosPoint {
    constructor(point) {
        this.origin = new Point(point.x, point.y);
        this.current = new Point(point.x, point.y);
    }
}