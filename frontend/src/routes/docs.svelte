<script lang="ts">
	import CenterContainer from '$lib/components/centerContainer.svelte';
</script>

<CenterContainer>
	<h1 class="text-4xl">Documentation</h1>
	<p>
		Shwitter is a very simple yet fast twitter clone. Going into this project i seeked a real
		challange. This is why i decided to build the project using only technologies i do not know.
	</p>
	<h2 class="text-2xl">Stack Comparison</h2>
	<p>
		Since i decided to build this project in such an unorthadox way let me first explain my usual
		stack.
	</p>
	<table class="w-full">
		<thead>
			<tr>
				<th>Usual</th>
				<th>Shwitter</th>
				<th>Stack location</th>
				<th>Role</th>
			</tr>
		</thead>
		<tbody>
			<tr>
				<td>Next js</td>
				<td>SvelteKit</td>
				<td>Frontend</td>
				<td>
					<abbr title="Server-side rendering">SSR</abbr>
					&mdash;
					<abbr title="Static site generation">SSG</abbr>
					/ Frontend</td
				>
			</tr>
			<tr>
				<td>React.js</td>
				<td>Svelte.js</td>
				<td>Frontend</td>
				<td>JS framework</td>
			</tr>
			<tr>
				<td>Emotion.js</td>
				<td>Tailwind CSS</td>
				<td>Frontend</td>
				<td>CSS framework</td>
			</tr>
			<tr>
				<td>Express.js</td>
				<td>Rocket.rs</td>
				<td>Backend</td>
				<td>Backend framework</td>
			</tr>
			<tr>
				<td>Mongo DB</td>
				<td>Redis</td>
				<td>DB</td>
				<td>Database</td>
			</tr>
			<tr>
				<td>Nx</td>
				<td>Make</td>
				<td>Devops</td>
				<td>Organization and build orchestration</td>
			</tr>
			<tr>
				<td>Docker</td>
				<td>Docker</td>
				<td>Devops</td>
				<td>Contaienrization and runtime standardization</td>
			</tr>
			<tr>
				<td>Heroku</td>
				<td>Heroku</td>
				<td>Devops</td>
				<td><abbr title="Platform as a service">PaaS</abbr></td>
			</tr>
		</tbody>
	</table>
</CenterContainer>
