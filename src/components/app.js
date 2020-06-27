import { h, Component, createRef } from 'preact';
import { Router } from 'preact-router';

// Code-splitting is automated for routes
import Home from '../routes/home';
import GOL from './gol';
import Flocking from './flocking';
import Chaos from './chaos';
import Navbar from './navbar';
import NavbarButton from './navbar-button';

export default class App extends Component {

    constructor() {
        super();
        this.container = createRef();
        this.state = { currentPage: 0, ticking: false };
        this.scrollTo = this.scrollTo.bind(this);
        this.handleScroll = this.handleScroll.bind(this);
    }

    handleScroll(event) {
		if (!this.state.ticking) {
			let newPage = this.state.currentPage;
			if (event.deltaY > 0) {
				newPage = (this.state.currentPage + 1) % 3;
			} else if (event.deltaY < 0) {
				const potentialPage = this.state.currentPage - 1;
				newPage = potentialPage < 0 ? 2 : potentialPage;
			}
			this.scrollTo(newPage);
		}
		event.preventDefault();
		event.stopPropagation();
    }

    scrollTo(page) {
		this.setState({ currentPage: page, ticking: true })
		document.querySelector(`#page${page}`).scrollIntoView();
		setTimeout(() => this.setState({ ticking: false }), 500);
    }

	render() {
        const handleNavClick = (page) => () => this.scrollTo(page);

		return (
			<div id="container" onWheel={this.handleScroll}>
                <div id="page0" />
				<GOL />
				<div id="page1" />
				<Flocking />
				<div id="page2" />
				<Chaos />
				<div id="app">
					<Home />
				</div>
				<div id="navbar">
					<Navbar>
						<NavbarButton icon="user" onClick={handleNavClick(0)} active={this.state.currentPage === 0} />
						<NavbarButton icon="archive" onClick={handleNavClick(1)} active={this.state.currentPage === 1} />
						<NavbarButton icon="at" onClick={handleNavClick(2)} active={this.state.currentPage === 2} />
					</Navbar>
				</div>
			</div>
		);
	}
}
