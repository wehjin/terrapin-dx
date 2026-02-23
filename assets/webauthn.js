// noinspection JSUnusedGlobalSymbols
export async function register_passkey_js(challenge_json) {
    try {
        const options = JSON.parse(challenge_json);
        console.log("Requesting WebAuthn Prompt with options:", options);
        const credential = await PublicKeyCredential.parseCreationOptionsFromJSON(options.publicKey);
        const result = await navigator.credentials.create({publicKey: credential});
        return JSON.stringify(result.toJSON());
    } catch (err) {
        console.error("WebAuthn Error:", err.name, err.message);
        throw new Error(err.name + ": " + err.message);
    }
}
