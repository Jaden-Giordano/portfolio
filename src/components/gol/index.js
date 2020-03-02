import {
    h,
    Component
} from 'preact';

import clearstags from './continuer';

const CELL_SIZE = 10;
const CELL_SPACING = 2;

const ALIVE = 10;

const SIM_TIME = 50;
const REPOPULATE = 100;

export default class GOL extends Component {
    componentWillMount() {
        this.width = window.innerWidth;
        this.height = window.innerHeight;
        this.cells = [
            [false]
        ];
        this.timer = 0;
    }

    mod = (x, n) => (x % n + n) % n;

    getNeighborsCount(x, y) {
        let count = 0;
        //console.log(y, this.cells[y])
        for (let i = -1; i <= 1; i++) {
            for (let j = -1; j <= 1; j++) {
                if (i === 0 && j === 0) continue;
                if (this.cells[this.mod(x + i, this.cells.length)][this.mod(y + j, this.cells[0].length)] === ALIVE)
                    count++;
            }
        }
        return count;
    }

    scale = (n, iMin, iMax, oMin, oMax) =>
        Math.floor((oMax - oMin) * ((n - iMin) / (iMax - iMin)) + oMin)

    getOpacity(weight) {
        if (weight === 0) return '00';
        return weight === ALIVE ? '' : (this.scale(weight, 0, ALIVE - 1, 0, 255)).toString(16);
    }

    resize() {

        if (this.canvas != null) {
            this.width = window.innerWidth-23;
            this.height = window.innerHeight - 4;
            this.canvas.width = this.width;
            this.canvas.height = this.height;

            const hCellCount = Math.ceil(this.width / (CELL_SIZE + CELL_SPACING));
            const vCellCount = Math.ceil(this.height / (CELL_SIZE + CELL_SPACING));

            const cells = new Array(hCellCount);

            for (let i = 0; i < hCellCount; i++) {
                cells[i] = new Array(vCellCount);
                for (let j = 0; j < vCellCount; j++) {
                    if (typeof this.cells[i] == 'undefined') {
                        cells[i][j] = Math.random() > 0.8 ? ALIVE : 0;
                        continue;
                    }
                    if (typeof this.cells[i][j] == 'undefined') {
                        cells[i][j] = Math.random() > 0.8 ? ALIVE : 0;
                        continue;
                    }
                    cells[i][j] = this.cells[i][j];
                }
            }

            this.cells = cells;
        }
    }

    update() {
        // only run sim if in view
        if (window.scrollY < this.height) {
            this.timer++;
           this.ctx.clearRect(0, 0, this.width, this.height);

            const next = new Array(this.cells.length);
            let livecells = 0;
            for (let i = 0; i < next.length; i++) {
                next[i] = new Array(this.cells[i].length);
                for (let j = 0; j < this.cells[i].length; j++) {
                    if (this.cells[i][j] === ALIVE && this.timer === REPOPULATE)
                        livecells++;

                    const neighborCount = this.getNeighborsCount(i, j);
                    if (neighborCount < 2 || neighborCount > 3) {
                        next[i][j] = this.cells[i][j] === ALIVE ? ALIVE - 1 : this.cells[i][j] - 1;
                        if (next[i][j] < 0) next[i][j] = 0;
                    } else if (neighborCount === 3) next[i][j] = ALIVE;
                    else if (neighborCount === 2) {
                        next[i][j] = this.cells[i][j] === ALIVE ? ALIVE : this.cells[i][j] - 1;
                        if (next[i][j] < 0) next[i][j] = 0;
                    }

                    this.ctx.beginPath();
                    this.ctx.rect(i * (CELL_SIZE + CELL_SPACING), j * (CELL_SIZE + CELL_SPACING), CELL_SIZE, CELL_SIZE);
                    this.ctx.fillStyle = `#73457C${this.getOpacity(this.cells[i][j])}`;
                    this.ctx.fill();
                }
            }

             if (this.timer > REPOPULATE) {
                this.timer = 0;
            this.cells = clearstags(next.slice());
            return;
            }
            this.cells = next.slice();
        }
    }

    componentDidMount() {
        this.ctx = this.canvas.getContext('2d');

        window.addEventListener('resize', () => {
            this.resize();
            this.update();
        });

        this.resize();

        this.updateInterval = setInterval(() => {
            this.update();
        }, SIM_TIME);
    }

    componentWillUnmount() {
        if (this.updateInterval) clearInterval(this.updateInterval);
    }

    render() {
        return <canvas ref = {
            c => this.canvas = c
        }
        />;
    }
}