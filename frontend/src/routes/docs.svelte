<script lang="ts">
	import Cataas from '$lib/components/cataas.svelte';
	import CenterContainer from '$lib/components/centerContainer.svelte';
	import Post from '$lib/components/post.svelte';
	import PostInput from '$lib/components/postInput.svelte';
	import Posts from '$lib/components/posts.svelte';

	import type { PublicPost } from '$lib/typings/api';

	let message = 'Post goes here';
	let i_like = false;

	let secret = false;
	let feed: PublicPost[] = [];

	$: post = {
		message,
		author: {
			self: '',
			data: { uuid: 'noid', name: 'Hello world' },
			error: null,
			refresh_token: null,
		},
		comment: null,
		comment_count: 15,
		i_like,
		created_at: 0,
		likes_count: 10,
		uuid: 'lol-this-does-not-exist',
	} as PublicPost;

	$: post, (feed[0] = post);
	$: message, (secret ||= message.toLocaleLowerCase() === 'cats are the best');

	$: gif = Math.random() > 0.75;
</script>

<CenterContainer>
	<div class="gap-4 flex flex-col">
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
					<td><abbr title="Session based cookie authentication">SBCA</abbr></td>
					<td><abbr title="Json Web Token">JWT</abbr></td>
					<td>Auth</td>
					<td>Authentication scheme</td>
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

		<h2>To create this project I made the following components.</h2>
		<h3 class="text-xl">Post component</h3>
		<p>
			The post component is at the heart of the project. It is the most common component and what
			the end user will se most.
		</p>
		<div class="pointer-events-none">
			<Post bind:post />
		</div>

		<h3 class="text-xl">Posts component</h3>
		<p>To display a collection of posts the Posts component is used</p>
		<div class="pointer-events-none"><Posts disableInvalidation disablePrefetch {feed} /></div>

		<h3 class="text-xl">Create post component</h3>
		<p>The second most common component in this project is the Author-post component.</p>
		<PostInput
			placeholder="Placeholder is changable"
			bind:value={message}
			on:submit={() => {
				feed = [
					...feed,
					{
						...post,
						uuid: `${Date.now()}`,
						i_like: Math.random() > 0.5,
						likes_count: Math.round(Math.random() * 99),
						comment_count: Math.round(Math.random() * 99),
					},
				];
				message = '';
			}}
		/>
		<p>
			A cool feature about this component is that you can submit with <kbd
				class="bg-pink-200 text-slate-900 rounded-sm p-0.5">Shift</kbd
			>
			+
			<kbd class="bg-pink-200 text-slate-900 rounded-sm p-0.5">Enter</kbd>.
			<br />
			Also, what are cats!?,
			<span
				class="inline-block bg-slate-900 select-none hover:bg-pink-200 text-slate-900 rounded-sm p-0.5"
				>Cats are ___ ____</span
			>
		</p>

		{#if secret}
			<h3 class="text-xl">CataaS</h3>
			<p>
				Cat as a service for all of your cat needs. You said i had to use an API? How about this:
				CataaS
			</p>

			<Cataas {gif} say={message} />
		{/if}
		<h2 class="text-2xl">features</h2>
		<p>
			using the above mentioned components i've implemented a plethora of features these vary from
			forest-comments to data searching. we will discuss some of these now.
		</p>
		<h3 class="text-xl">user sessions</h3>
		<p>
			user sessions was one of the most complex features to implement. this is because i chose the
			incorrect session management scheme from the get go. this was the 'cookie session' scheme.
			this has numerous flaws:
		</p>
		<ol class="list-decimal pl-10">
			<li>it takes up data-volume on the server</li>
			<li>
				it is slow since you need more than one database <br />
				requests to access the session data
			</li>
			<li>it is limiting for cross origin requests</li>
		</ol>
		<p>
			problems 1 & 2 are not that critical. but problem 3 was the pest. to fix all these problems i
			chose to use jwt tokens to store user data. this is a far supperior option in my opinion.
			using these jwt tokens i then enabled the user to do full authentication to the app.
		</p>
		<p />
		<h3 class="text-xl">posting</h3>
		<p>
			posting is the most trivial feature of the app. posts are stored as roots of trees in an
			orderd forest on the database. you post posts logically enugth by writing a post in the post
			input and then clicking send. simple. easy. clean.
		</p>
		<h3 class="text-xl">orderd-child trees comments.</h3>
		<p>
			child comments are stored in an orderd tree bellow the top level root of the application. the
			post. you can comment on all posts and all comments. these are then resolved in o(n) time from
			the server using a simple indexing strategy.
		</p>
		<h3 class="text-xl">hash-set likes.</h3>
		<p>
			the server also stores likes for each post/comment. these are stored as a hash-set making it
			fast (o(1)) and easy to like posts. to like a post you juts press the heart icon on a post or
			comment and voiala, a like.
		</p>
		<h3 class="text-xl">searching</h3>
		<p>
			searching is one of the worst features of the app but implemented through neceity. there is
			support for fuzzy text matching but i have disabled this since it would mean stopwords would
			be skipped and that would be anoying. you search using the search bar. this is done through a
			simple form submission. i am too lazy to implement anything else.
		</p>
		<h3 class="text-xl">post view</h3>
		<p>by clicking on the text in a post it will redirect you to the posts own page.</p>
		<h3 class="text-xl">user view</h3>
		<p>
			by clicking on the text in the username in a post iy will redirect you to the users own page.
		</p>
		<h2 class="text-2xl">non-features</h2>
		<p>
			there are a few features i wanted to implement but through technical limitations were unable
			to. here are some of them.
		</p>
		<h3 class="text-xl">ip locked jwt tokens</h3>
		<p>
			i would hash the ip and store it as the aud field on the jwt. this would be neet but not
			necessary. it would prevent cookie theatht.
		</p>
		<h3 class="text-xl">file uploads</h3>
		<p>i dont want to spend money on s3 for a simple school project.</p>
	</div>
</CenterContainer>
