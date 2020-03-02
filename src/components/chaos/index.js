import { Component } from 'preact';
import style from './style.css';

import { ChaosPoint } from './math';
import { Point } from '../../utils/math';
import equations from './equations';

const SIM_TIME = 5;
const POINTS = 10000;
const SCALE = 1000;

export default class Chaos extends Component {
  resize() {
    if (this.canvas != null) {
      this.width = window.innerWidth - 23;
      this.height = window.innerHeight;
      this.canvas.width = this.width;
      this.canvas.height = this.height;
    }
  }

  componentDidMount() {
    this.ctx = this.canvas.getContext('2d');
    this.ctx.clearRect(0, 0, this.width, this.height);
    this.ctx.fillStyle = "#202020"
    this.ctx.fillRect(0, 0, this.width, this.height);

    window.addEventListener('resize', () => {
      this.resize();
      this.update();
    });

    this.resize();

    //this.timer = Math.random() * 6 - 3;
    this.timer = 0.01;
    this.equation = equations[0];
    this.points = [];
    this.points[0] = new Point(this.timer, this.timer)
    for (let i = 1; i < POINTS; i += 1) {
      this.points[i] = new Point(this.equation(this.points[i - 1], this.timer));
    }

    this.updateInterval = setInterval(() => {
      this.timer = this.timer + 0.00001;
      this.update();
    }, SIM_TIME);

    // this.canvas.addEventListener('click', (evt) => {
    //     this.timer = this.timer + 0.01;
    //     if (this.timer > 10) this.timer = -10;
    //     this.update();
    //     console.log(this.points);
    // });
  }

  componentWillUnmount() {
    if (this.updateInterval) clearInterval(this.updateInterval);
  }

  update() {
    if (window.scrollY < this.height * 3 && window.scrollY > this.height) {
      // this.ctx.clearRect(0, 0, this.width, this.height);
      this.ctx.fillStyle = "#20202051";
      this.ctx.fillRect(0, 0, this.width, this.height)
      this.points[0].x = this.timer;
      this.points[0].y = this.timer;
        for (let i = 1; i < POINTS; i += 1) {
          this.ctx.save();
    this.ctx.translate(this.width / 2, this.height / 2);
          const point = this.points[i];
          this.ctx.fillStyle = '#73457CFF';
          this.ctx.fillRect((point.x * SCALE), (point.y * SCALE), 2, 2)

          this.points[i]= this.equation(this.points[i-1], this.timer);
          this.ctx.restore();
        }
    }
  }

  render() {
    return <canvas class = {
      style.chaos
    }
    ref = {
      c => this.canvas = c
    }
      />;
  }
}
