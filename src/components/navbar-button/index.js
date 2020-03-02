import { h, Component } from 'preact';
import classNames from 'classnames';

import Icon from '../icon';

import style from './style.css';

const NavbarButton = ({ icon = "user", active = false }) => {
    return (
        <div class={style.navbarbutton}>
            <div class={classNames(style.marker, active ? style.active : style.inactive)} />
            <Icon icon={icon} class="white" />
        </div>
    );
}

export default NavbarButton;
