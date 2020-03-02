import { h, Component } from 'preact';

import style from './style.css';

const Navbar = ({ children }) => {
    return (
        <div class={style.navbar}>
            {children}
        </div>
    );
}

export default Navbar;