require('bootstrap');
import $ from "jquery";
import './app.scss';

const vw = Math.max(document.documentElement.clientWidth || 0, window.innerWidth || 0)
const vh = Math.max(document.documentElement.clientHeight || 0, window.innerHeight || 0)


// Add opacity fade in on top navbar as user scrolls down. Limit is the height in which it becomes fully viewable
let limit = 1000;
$(window).on('scroll', function() {
    var st = $(this).scrollTop();
    if (st < limit) {
        $(".nav").css({'opacity' : (0 + st/limit)});
    } else {$(".nav").css({'opacity': 1})}
})