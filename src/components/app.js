import { h, Component } from 'preact';
import { Router } from 'preact-router';

// Code-splitting is automated for routes
import Home from '../routes/home';
import GOL from './gol';
import Flocking from './flocking';
import Chaos from './chaos';
import Navbar from './navbar';
import NavbarButton from './navbar-button';

export default class App extends Component {
	
	/** Gets fired when the route changes.
	 *	@param {Object} event		"change" event from [preact-router](http://git.io/preact-router)
	 *	@param {string} event.url	The newly routed URL
	 */
	handleRoute = e => {
		this.currentUrl = e.url;
	};

	render() {
		return (
			<div id="container">
				<GOL />
				<Flocking />
				<Chaos />
				<div id="navbar">
					<Navbar>
						<NavbarButton icon="user" active={true} />
						<NavbarButton icon="archive" />
						<NavbarButton icon="at" />
					</Navbar>
				</div>
				<div id="app">
					<Router onChange={this.handleRoute}>
						<Home path="/" />
					</Router>
				</div>
			</div>
		);
	}
}
