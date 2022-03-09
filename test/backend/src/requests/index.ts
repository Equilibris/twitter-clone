import { sleep } from 'k6'

export const invoke =
	(getInterval: () => number = () => Math.random() * 5, request: () => void) =>
	() => {
		sleep(getInterval())
		request()
	}
