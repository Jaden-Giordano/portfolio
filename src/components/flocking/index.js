import {
    h,
    Component
} from 'preact';
import Victor from './victor.min';
import style from './style.css';
import { Quadtree, Rectangle, Point, Circle } from './quadtree.js';
const SIM_TIME = 25;
import Boid from './boid';

export default class Flocking extends Component {
    componentWillMount() {
        this.boids = [];
        this.mouse = new Victor(0, 0);
        this.timer = 0;
        this.points = [];
    }

    resize() {

        if (this.canvas != null) {
            this.width = window.innerWidth - 23;
            this.height = window.innerHeight;
            this.canvas.width = this.width;
            this.canvas.height = this.height;
        }
    }



    update() {
        if (window.scrollY < this.height * 2 && window.scrollY > 0) {
            //this.ctx.clearRect(0, 0, this.width, this.height);
            this.ctx.fillStyle = "#202020"
            this.ctx.fillRect(0, 0, this.width, this.height);
            this.quadtree = new Quadtree(2, new Rectangle(0, 0, this.width, this.height));

            let tempcopy = this.boids.slice(0);

            for (const b of this.boids){
                this.quadtree.insert(b);
            }

            this.quadtree.draw(this.ctx);
            for (let boid of this.boids) {
                boid.flock(tempcopy, this.mouse, this.quadtree);
                boid.update();
                boid.show();

            }


        }
    }

    componentDidMount() {
        this.ctx = this.canvas.getContext('2d');
        this.ctx.clearRect(0, 0, this.width, this.height);

        window.addEventListener('resize', () => {
            this.resize();
            this.update();
        });

        this.canvas.addEventListener('click', (evt) => {
            let rect = this.canvas.getBoundingClientRect();
            this.mouse = Victor(evt.clientX - rect.left,
                evt.clientY - rect.top);

        });

        this.canvas.addEventListener('mousemove', (evt) => {
            let rect = this.canvas.getBoundingClientRect();
            this.mouse = Victor(evt.clientX - rect.left,
                evt.clientY - rect.top);
        });


        this.resize();

      console.log(this.width / 100 * this.height / 100);

        for (let i = 0; i < Math.floor(this.width / 100 * this.height / 100); i++) {;
            this.boids.push(new Boid(this.canvas, this.ctx));
        }


        this.updateInterval = setInterval(() => {
            this.update();
        }, SIM_TIME);

        this.quadtree = new Quadtree(4, new Rectangle(0, 0, this.width, this.height));

        //for(let i = 0; i < 30; i++){
        //    this.quadtree.insert(new Point(Math.random()*this.width, Math.random()*this.height));
        //}
    }

    componentWillUnmount() {
        if (this.updateInterval) clearInterval(this.updateInterval);
    }


    render() {
        return <canvas class={
            style.flocking
        }
            ref={
                c => this.canvas = c
            }
        />;
    }
}
