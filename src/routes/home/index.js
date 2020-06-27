import { h } from 'preact';
import style from './style';
import Icon from '../../components/icon';

const Home = () => (
	<div class={style.home}>
		<h1 class={style.title}>Jaden Giordano</h1>
		<hr size="1" />
		<div class={style.list}>
			<span class="orange">Frontend</span>
			<Icon icon="circle" />
			<span class="green">Design Systems</span>
			<Icon icon="circle" size={4} />
			<span class="blue">HTML/JS/CSS</span>
		</div>
	</div>
);

export default Home;
