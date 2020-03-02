import { h } from 'preact';
import style from './style';
import Icon from '../../components/icon';

const Home = () => (
	<div class={style.home}>
		<h1 class={style.title}>Aidan Chelig</h1>
		<hr size="1"></hr>
		<div class={style.list}>
			<span class="orange">Fullstack</span>
			<Icon icon="circle" />
			<span class="green">Frontend</span>
			<Icon icon="circle" size={4} />
			<span class="blue">Javascript</span>
		</div>
	</div>
);

export default Home;
