import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// cf. https://github.com/sveltejs/svelte-preprocess
	preprocess: preprocess(),

	kit: {
		adapter: adapter()
	}
};

export default config;
