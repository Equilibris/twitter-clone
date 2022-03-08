import { Options } from 'k6/options'
import { signUp } from './requests/auth'

const count = 1000

export let options: Options = {
	stages: [
		{ target: count, duration: '10s' },
		{ target: count, duration: '20s' },
		{ target: 5 * count, duration: '5s' },
		{ target: 5 * count, duration: '7.5s' }, // Spike
		{ target: count, duration: '2s' },
		{ target: count, duration: '20s' },
		{ target: 0, duration: '5s' },
	],
	thresholds: {
		http_req_failed: ['rate<0.05'],
		http_req_duration: ['p(90)<10000'],
		errors: ['rate<0.1'],
	},
}
export default signUp(() => 0.5 + 2 * Math.random())
