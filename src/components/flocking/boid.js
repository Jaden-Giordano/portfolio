import Victor from './victor.min';
import {Circle} from '../../utils/math';
import {
    isContext
} from 'vm';

export default class Boid {
    constructor(canvas, ctx) {
        this.ctx = ctx;
        this.canvas = canvas;
        this.position = new Victor().randomize(new Victor(0, 0), new Victor(this.canvas.width, this.canvas.height));
        this.velocity = new Victor((Math.random() - 0.5) * 10, (Math.random() - 0.5) * 10);
        this.acceleration = new Victor(0, 0);
        this.alignmentForce = 0.2;
        this.cohesionForce = 0.2;
        this.seperationForce = 0.6;
        this.perceptionSize = 100;
        this.maxSpeed = 7/2;
        this.velocity = this.setMag(this.maxSpeed, this.velocity);
    }

    align(boids) {
        let perception = 75/2;
        let steering = new Victor(0, 0);
        let total = 0;
        for (let other of boids) {
            let d = this.wrappedDistance(this.position, other.position);
            if (d < perception && other != this) {
                steering.add(other.velocity);
                total++;
            }
        }
        if (total > 0 && steering != new Victor(0, 0)) {
            steering.divide(new Victor(total, total));
            steering = this.setMag(this.maxSpeed, steering);
            steering.subtract(this.velocity);
            steering = this.limit(steering, this.alignmentForce);
        }
        return steering;
    }

    cohesion(boids) {
        let perception = 75/2;
        let steering = new Victor(0, 0);
        let total = 0;
        for (let other of boids) {
            let d = this.wrappedDistance(this.position, other.position);
            if (d < perception && other != this) {
                steering.add(other.position);
                total++;
            }
        }
        if (total > 0 && steering != new Victor(0, 0)) {
            steering.divide(new Victor(total, total));
            steering.subtract(this.position);
            steering = this.setMag(this.maxSpeed, steering);
            steering.subtract(this.velocity);
            steering = this.limit(steering, this.cohesionForce);

        }
        return steering;
    }

    seperation(boids) {
        let perception = 50/2;
        let steering = new Victor(0, 0);
        let total = 0;
        for (let other of boids) {
            let d = this.wrappedDistance(this.position, other.position);
            if (d < perception && other != this) {
                let diff = this.position.clone();
                diff = diff.subtract(other.position);
                diff.divide(new Victor(d, d));
                steering.add(diff);
                total++;
            }
        }
        if (total > 0) {
            steering.divide(new Victor(total, total));
            steering = this.setMag(this.maxSpeed, steering);
            steering.subtract(this.velocity);
            steering = this.limit(steering, this.seperationForce);
        }
        return steering;
    }


    mouse(mcoords) {
        let perception = 100;
        let steering = new Victor(0, 0);
        let total = 0;
        let d = this.wrappedDistance(this.position, mcoords);
        if (d < perception) {
            steering.add(mcoords);
            total++;
        }
        if (total > 0 && steering != new Victor(0, 0)) {
            steering.divide(new Victor(total, total));
            steering.subtract(this.position);
            steering = this.setMag(this.maxSpeed, steering);
            steering.subtract(this.velocity);
            steering = this.limit(steering, .5);

        }
        return steering;
    }


    flock(boids, mousecoords, qt) {
        this.qt = qt;
        this.acceleration.multiply(new Victor(0, 0));
        let percievedBoids = this.getLocalBoids(new Circle(this.position.x, this.position.y, this.perceptionSize));
        let alignment = this.align(percievedBoids);
        let cohesion = this.cohesion(percievedBoids);
        let seperation = this.seperation(percievedBoids);
        let mousemove = this.mouse(percievedBoids);
        this.acceleration.add(seperation).add(cohesion).add(alignment).add(mousemove);
    }

    wrappedDistance(vec1, vec2) {
        let dx = vec1.distanceX(vec2);
        let dy = vec1.distanceY(vec2);

        if (dx > this.canvas.width / 2)
            dx = this.canvas.width - dx;

        if (dy > this.canvas.height / 2)
            dy = this.canvas.height - dy;


        return Math.sqrt(dx * dx + dy * dy) + 0.00001;
    }

    getLocalBoids(circ){
        let arr = [];
        if(circ.x + circ.r > this.canvas.width)
            this.qt.query(new Circle(circ.x - this.canvas.width, circ.y, circ.r), arr);

        if(circ.x - circ.r < 0)
            this.qt.query(new Circle(circ.x + this.canvas.width, circ.y, circ.r), arr);

        if(circ.y - circ.r < 0)
            this.qt.query(new Circle(circ.x, circ.y+this.canvas.height, circ.r), arr);

        if(circ.y + circ.r < this.canvas.height)
            this.qt.query(new Circle(circ.x, circ.y-this.canvas.height, circ.r), arr);
        this.qt.query(circ, arr);

        return arr;
    }

    edges() {
        if (this.position.x > this.canvas.width + 11) {
            this.position.x = -11;
        } else if (this.position.x < -11) {
            let newx = this.canvas.width + 11;
            if(newx != NaN)
            this.position.x = newx;
        }

        if (this.position.y > this.canvas.height + 11) {
            this.position.y = -11;
        } else if (this.position.y < -11) {
            let newy = this.canvas.height + 11;
            if(newy != NaN)
            this.position.y = newy;
        }

    }

    update() {
        this.position.add(this.velocity);
        this.velocity.add(this.acceleration);
        this.velocity = this.limit(this.velocity, this.maxSpeed);
        this.velocity = this.setMag(this.maxSpeed, this.velocity);
        this.edges();

    }

    setMag(magnitude, vec) {
        let currentMag = vec.magnitude();
        let newmag = new Victor(0, 0);
        if (currentMag != 0) {
            newmag = vec.multiply(new Victor(magnitude, magnitude)).divide(new Victor(currentMag, currentMag));
        }
        return newmag;
    }

    limit(vec, speed) {
        if (vec.magnitude() > speed) {
            return this.setMag(speed, vec);
        } else
            return vec;
    }

    show() {
        this.ctx.translate(this.position.x, this.position.y);

        this.ctx.rotate(this.velocity.direction());
        this.ctx.beginPath();
        this.ctx.moveTo(0, 0);
        this.ctx.lineTo(-10, -2.5);
        this.ctx.lineTo(-10, 2.5);
        this.ctx.closePath();

        this.ctx.lineWidth = 10;
        this.ctx.strokeStyle = '#9b3dadFF';
        this.ctx.stroke();

        this.ctx.fillStyle = '#73457CFF';

        //this.ctx.shadowColor = '#111';
        //this.ctx.shadowBlur = 5;
        //this.ctx.shadowOffsetX = 5;
        //this.ctx.shadowOffsetY = 5;

        this.ctx.fill();
        this.ctx.closePath();
        this.ctx.setTransform(1, 0, 0, 1, 0, 0); // restore default transform
    }
}
