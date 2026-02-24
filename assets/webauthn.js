// noinspection JSUnusedGlobalSymbols
export async function register_passkey_js(challenge_json) {
    try {
        const options = JSON.parse(challenge_json);
        const publicKey = await PublicKeyCredential.parseCreationOptionsFromJSON(options.publicKey);
        const credential = await navigator.credentials.create({publicKey: publicKey});
        return JSON.stringify(credential.toJSON());
    } catch (err) {
        throw new Error(err.name + ": " + err.message);
    }
}

export async function authenticate_passkey_js(challenge_json) {
    try {
        const options = JSON.parse(challenge_json);
        const publicKey = PublicKeyCredential.parseRequestOptionsFromJSON(options.publicKey);
        const assertion = await navigator.credentials.get({publicKey});
        return JSON.stringify(assertion.toJSON());
    } catch (err) {
        throw new Error(err.name + ": " + err.message);
    }
}