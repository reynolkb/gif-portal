const anchor = require('@project-serum/anchor');

const main = async () => {
	console.log('Starting tests...');

	const provider = anchor.Provider.env();
	anchor.setProvider(provider);
	const program = anchor.workspace.Gifportal;

	const baseAccount = anchor.web3.Keypair.generate();
	console.log(baseAccount);
	// startStuffOff is camelCase in javascript because anchor does this for us so we can follow the best practices regardless of what language we are using
	const tx = await program.rpc.startStuffOff({
		accounts: {
			baseAccount: baseAccount.publicKey,
			user: provider.wallet.publicKey,
			systemProgram: anchor.web3.SystemProgram.programId,
		},
		signers: [baseAccount],
	});
	console.log('Your transaction signature', tx);

	let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
	console.log('GIF Count', account.totalGifs.toString());

	await program.rpc.addGif('https://media.giphy.com/media/ZFFVNwIbjsKtP3lHYK/giphy-downsized-large.gif', {
		accounts: {
			baseAccount: baseAccount.publicKey,
			user: provider.wallet.publicKey,
		},
	});

	account = await program.account.baseAccount.fetch(baseAccount.publicKey);
	console.log('GIF Count', account.totalGifs.toString());
	console.log('GIF List', account.gifList);
};

const runMain = async () => {
	try {
		await main();
		process.exit(0);
	} catch (error) {
		console.error(error);
		process.exit(1);
	}
};

runMain();
