#![feature(plugin, proc_macro, proc_macro_non_items)]
#![plugin(rocket_codegen)]

extern crate maud;
extern crate rocket;

use maud::{DOCTYPE, html, Markup};
use rocket::Request;

#[get("/")]
fn index() -> Markup {
    template_base("Home", html! {
        style {
            "*{margin:0;padding:0;}"
            "body{background:#222;}"
            "canvas{position:absolute;border:0 solid;box-shadow:inset 0 0 80px #4162a9;transform:translateZ(0);width:100%;height:100%;}"
        }
        canvas width="1000" height="1000" {}
        script type="text/javascript" src="particles.js" {}
    })
}

//window.onload = function() {
//var   canvas = document.querySelector('canvas'),
//         ctx = canvas.getContext('2d'),
//   particles = [],
//           w = window.innerWidth,
//           h = window.innerHeight,
//patriclesNum = w * h / 20000,
//        ease = 01,
//      colors = ['#f35d4f','#b08ee3','#b5d98c','#6ddaf1','#f1e85b'];
//
//canvas.width = w;
//canvas.height = h;
//
//ctx.globalCompositeOperation = 'lighter';
//ctx.linewidth = 0.5;
//
//function Factory() {
//  var spd = 2.0;
//  this.x =  Math.round(Math.random() * w);
//  this.y =  Math.round(Math.random() * h);
//  this.rad = Math.round(Math.random() * 5) + 2;
//  this.rgba = colors[Math.floor(Math.random() * colors.length)];
//  this.vx = br(spd);
//  this.vy = br(spd);
//  this.ex = br(ease) / w;
//  this.ey = br(ease) / h;
//}
//
//function draw(particles) {
//  for (var i = 0; i < particles.length; i++) {
//    var temp = particles[i];
//
//    for (var j = i; j < particles.length; j++) {
//       var temp2 = particles[j];
//
//       if (findDistance(temp, temp2) < 50) {
//          ctx.strokeStyle = temp.rgba;
//          ctx.beginPath();
//          ctx.moveTo(temp.x, temp.y);
//          ctx.lineTo(temp2.x, temp2.y);
//          ctx.stroke();
//       }
//    }
//
//    ctx.fillStyle = temp.rgba;
//    ctx.strokeStyle = temp.rgba;
//
//    ctx.beginPath();
//    ctx.arc(temp.x, temp.y, temp.rad, 0, Math.PI*2, true);
//    ctx.fill();
//    ctx.closePath();
//
//    ctx.beginPath();
//    ctx.arc(temp.x, temp.y, temp.rad+5, 0, Math.PI*2, true);
//    ctx.stroke();
//    ctx.closePath();
//
//    temp.x += temp.vx;
//    temp.y += temp.vy;
//    temp.vx += temp.ex;
//    temp.vy += temp.ey;
//
//    if (temp.x > w) { temp.x = 0; temp.ex = br(ease) / w; }
//    if (temp.x < 0) { temp.x = w; temp.ex = br(ease) / w; }
//    if (temp.y > h) { temp.y = 0; temp.ey = br(ease) / h; }
//    if (temp.y < 0) { temp.y = h; temp.ey = br(ease) / h; }
//  }
//}
//
//function findDistance(p1,p2){
//  return Math.sqrt( Math.pow(p2.x - p1.x, 2) + Math.pow(p2.y - p1.y, 2) );
//}
//
//function br(amp) {
//  return Math.random() * amp - amp / 2;
//}
//
//window.requestAnimFrame = (function(){
//  return  window.requestAnimationFrame       ||
//          window.webkitRequestAnimationFrame ||
//          window.mozRequestAnimationFrame    ||
//          function( callback ){
//            window.setTimeout(callback, 1000 / 30);
//          };
//})();
//
//(function init(){
//  for(var i = 0; i < patriclesNum; i++){
//    var p = new Factory;
//    particles[p.rgba] = particles[p.rgba] || [];
//    particles[p.rgba].push(p);
//  }
//})();
//
//(function loop(){
//  ctx.clearRect(0, 0, w, h);
//  for (var i = 0; i < colors.length; i++) {
//    draw(particles[colors[i]]);
//  }
//  requestAnimFrame(loop);
//})();
//}
#[get("/particles.js")]
fn script() -> &'static str {
    r##"window.onload=function(){var t=document.querySelector("canvas"),e=t.getContext("2d"),a=[],n=window.innerWidth,r=window.innerHeight,o=n*r/2e4,i=1,h=["#f35d4f","#b08ee3","#b5d98c","#6ddaf1","#f1e85b"];function d(){this.x=Math.round(Math.random()*n),this.y=Math.round(Math.random()*r),this.rad=Math.round(5*Math.random())+2,this.rgba=h[Math.floor(Math.random()*h.length)],this.vx=y(2),this.vy=y(2),this.ex=y(i)/n,this.ey=y(i)/r}function s(t){for(var a=0;a<t.length;a++){for(var o=t[a],h=a;h<t.length;h++){var d=t[h];s=o,l=d,Math.sqrt(Math.pow(l.x-s.x,2)+Math.pow(l.y-s.y,2))<50&&(e.strokeStyle=o.rgba,e.beginPath(),e.moveTo(o.x,o.y),e.lineTo(d.x,d.y),e.stroke())}e.fillStyle=o.rgba,e.strokeStyle=o.rgba,e.beginPath(),e.arc(o.x,o.y,o.rad,0,2*Math.PI,!0),e.fill(),e.closePath(),e.beginPath(),e.arc(o.x,o.y,o.rad+5,0,2*Math.PI,!0),e.stroke(),e.closePath(),o.x+=o.vx,o.y+=o.vy,o.vx+=o.ex,o.vy+=o.ey,o.x>n&&(o.x=0,o.ex=y(i)/n),o.x<0&&(o.x=n,o.ex=y(i)/n),o.y>r&&(o.y=0,o.ey=y(i)/r),o.y<0&&(o.y=r,o.ey=y(i)/r)}var s,l}function y(t){return Math.random()*t-t/2}t.width=n,t.height=r,e.globalCompositeOperation="lighter",e.linewidth=.5,window.requestAnimFrame=window.requestAnimationFrame||window.webkitRequestAnimationFrame||window.mozRequestAnimationFrame||function(t){window.setTimeout(t,1e3/30)},function(){for(var t=0;t<o;t++){var e=new d;a[e.rgba]=a[e.rgba]||[],a[e.rgba].push(e)}}(),function t(){e.clearRect(0,0,n,r);for(var o=0;o<h.length;o++)s(a[h[o]]);requestAnimFrame(t)}()};"##
}

#[error(404)]
fn not_found(req: &Request) -> Markup {
    template_base("404", html! {
        h1 "404: Hey! There's nothing here."
        "The page at " (req.uri().as_str()) " does not exist!"
    })
}

fn template_base(title: &str, markup: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                title (title)
            }
            body (markup)
        }
    }
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, script])
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}
