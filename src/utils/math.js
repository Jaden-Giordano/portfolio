export class Point {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
}

export class Circle {
  constructor(x, y, r) {
    this.x = x;
    this.y = y;
    this.r = r;
  }

  draw(ctx) {
    ctx.beginPath();
    ctx.arc(this.x, this.y, this.r, 0, 2 * Math.PI, false);
    ctx.stroke();
    ctx.closePath();

  }

  contains(point) {
    return Math.sqrt(Math.pow(point.x - this.x, 2) + Math.pow(point.y - this.y, 2)) <= this.r;
  }
}

export class Rectangle {
  constructor(x, y, w, h) {
    this.x = x;
    this.y = y;
    this.w = w;
    this.h = h;
  }

  intersectCircle(circle) {
    if(this.contains(new Point(circle.x, circle.y)))
      return true;
    let testx = circle.x;
    let testy = circle.y;
    if (circle.x < this.x) testx = this.x;
    else if (circle.x > this.x + this.w) testx = this.x + this.w;

    if (circle.y < this.y) testy = this.y;
    else if (circle.y > this.y + this.h) testy = this.y + this.h;

    let distx = circle.x - testx;
    let disty = circle.y - testy;
    let dist = Math.sqrt((distx * distx) + (disty * disty));
    if (dist <= circle.r) {
      return true;
    }
    return false;
  }

  intersectSquare(s, x, y){
      if(this.x > x+s || this.x < x-s) return false;
      if(this.y > y+s || y-s > this.y + this.h) return false;
      return true;
  }

  contains(point) {
    return (point.x >= this.x && this.w + this.x > point.x && point.y >= this.y && this.h + this.y > point.y);
  }

}