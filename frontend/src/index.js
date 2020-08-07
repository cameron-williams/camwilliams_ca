require('bootstrap');
import './app.scss';

export const add = (a, b) => {
    return a + b;
   }


let x = add(1, 3);

console.log("Hi there: " + x);