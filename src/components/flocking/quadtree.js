import {Point, Rectangle, Circle} from '../../utils/math';

class Quadtree {
  constructor(capacity, rectangle) {
    this.rect = rectangle;
    this.cap = capacity;
    this.points = [];
    this.divided = false;
  }

  subdivide() {
    let nw = new Rectangle(this.rect.x, this.rect.y, this.rect.w / 2, this.rect.h / 2);
    this.northwest = new Quadtree(this.cap, nw);
    let sw = new Rectangle(this.rect.x, this.rect.y + this.rect.h / 2, this.rect.w / 2, this.rect.h / 2);
    this.southwest = new Quadtree(this.cap, sw);
    let ne = new Rectangle(this.rect.x + this.rect.w / 2, this.rect.y, this.rect.w / 2, this.rect.h / 2);
    this.northeast = new Quadtree(this.cap, ne);
    let se = new Rectangle(this.rect.x + this.rect.w / 2, this.rect.y + this.rect.h / 2, this.rect.w / 2, this.rect.h / 2);
    this.southeast = new Quadtree(this.cap, se);


  }

  insert(boid) {

    if (!this.rect.contains(boid.position)) {
      return false;
    }

    if (this.points.length < this.cap) {
      this.points.push(boid);
      return true;
    } else {
      if (!this.divided) {
        this.subdivide();
        this.divided = true;
      }
      if (this.northwest.insert(boid)) {
        return true;
      } else if (this.northwest.insert(boid)) {
        return true;
      } else if (this.northeast.insert(boid)) {
        return true;
      } else if (this.southeast.insert(boid)) {
        return true;
      } else if (this.southwest.insert(boid)) {
        return true;
      }
    }
  }

  draw(ctx) {
    ctx.beginPath()
    ctx.rect(this.rect.x, this.rect.y, this.rect.w, this.rect.h);
    ctx.lineWidth = 2;
    ctx.strokeStyle = '#101010';
    ctx.stroke();
    ctx.closePath();
    if (this.divided) {
      this.northwest.draw(ctx);
      this.northeast.draw(ctx);
      this.southeast.draw(ctx);
      this.southwest.draw(ctx);
    }
  }

querySquare(x, y, s, arr) {
    let found = [];
    if(arr){
      found = arr;
    }

    if (!this.rect.intersectSquare(s, x, y)) {
      return;
    } else {
      for (let p of this.points) {
        if (circle.contains(p.position)) {
          found.push(p);
        }
      }

      if (this.divided) {
        this.northeast.querySquare(x, y, s, found);
        this.northwest.querySquare(x, y, s, found);
        this.southeast.querySquare(x, y, s, found);
        this.southwest.querySquare(x, y, s, found);
      }

      return found;
    }
  }
  query(circle, arr) {
    let found = [];
    if(arr){
      found = arr;
    }

    if (!this.rect.intersectCircle(circle)) {
      return;
    } else {
      for (let p of this.points) {
        if (circle.contains(p.position)) {
          found.push(p);
        }
      }

      if (this.divided) {
        this.northeast.query(circle, found);
        this.northwest.query(circle, found);
        this.southeast.query(circle, found);
        this.southwest.query(circle, found);
      }

      return found;
    }

  }


}

export {
  Quadtree,
  Rectangle,
  Point,
  Circle
}