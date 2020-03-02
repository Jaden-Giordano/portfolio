import { h, createElement } from 'preact';
import convert from './converter';
import { icon, parse } from '@fortawesome/fontawesome-svg-core';

const normalizeIconArgs = icon => {
	if (icon === null) {
		return null;
	}

	if (typeof icon === 'object' && icon.prefix && icon.iconName) {
		return icon;
	}

	if (Array.isArray(icon) && icon.length === 2) {
		return { prefix: icon[0], iconName: icon[1] };
	}

	if (typeof icon === 'string') {
		return { prefix: 'fas', iconName: icon };
	}
};

const objectWithKey = (key, value) => (Array.isArray(value) && value.length > 0) ||
        (!Array.isArray(value) && value)
	? { [key]: value }
	: {};

const classList = props => {
	let classes = {
		'fa-spin': props.spin,
		'fa-pulse': props.pulse,
		'fa-fw': props.fixedWidth,
		'fa-border': props.border,
		'fa-li': props.listItem,
		'fa-flip-horizontal':
            props.flip === 'horizontal' || props.flip === 'both',
		'fa-flip-vertical': props.flip === 'vertical' || props.flip === 'both',
		[`fa-${props.size}`]: props.size !== null,
		[`fa-rotate-${props.rotation}`]: props.rotation !== null,
		[`fa-pull-${props.pull}`]: props.pull !== null
	};

	return Object.keys(classes)
		.map(key => (classes[key] ? key : null))
		.filter(key => key);
};

const Icon = props => {
	const { icon: iconArgs, mask: maskArgs, symbol, class: className } = props;
	const normalIcon = normalizeIconArgs(iconArgs);
	const classes = objectWithKey('classes', [
		...classList(props),
		...className.split(' ')
	]);
	const transform = objectWithKey(
		'transform',
		typeof props.transform === 'string'
			? parse.transform(props.transform)
			: props.transform
	);
	const mask = objectWithKey('mask', normalizeIconArgs(maskArgs));
	const renderedIcon = icon(normalIcon, {
		...classes,
		...transform,
		...mask,
		symbol
	});

	const { abstract } = renderedIcon;
	const convertCurry = convert.bind(null, createElement);
	const extraProps = {};

	Object.keys(props).forEach(key => {
		if (!Icon.defaultProps.hasOwnProperty(key)) extraProps[key] = props[key];
	});

	return convertCurry(abstract[0], extraProps);
};

Icon.defaultProps = {
	border: false,
	class: '',
	mask: null,
	fixedWidth: false,
	flip: null,
	icon: null,
	listItem: false,
	pull: null,
	pulse: false,
	name: '',
	rotation: null,
	size: null,
	spin: false,
	symbol: false,
	transform: null
};

export default Icon;
