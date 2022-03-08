import { Options } from 'k6/options'
import { signUp } from './requests/auth'

export let options: Options = {
	scenarios: {
		smokeA: {
			vus: 1,
			duration: '10s',
			executor: 'constant-vus',
		},
		smokeB: {
			vus: 10,
			duration: '20s',
			executor: 'constant-vus',
		}
	},
	thresholds: {
		http_req_failed: ['rate<0.01'],
		http_req_duration: ['p(99)<250'],
		errors: ['rate<0.1'],
	},
}
export default signUp(() => 0.3 * Math.random())