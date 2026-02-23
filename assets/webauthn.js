// noinspection JSUnusedGlobalSymbols
export function register_passkey_js(challenge_json) {
    return new Promise(async (resolve, reject) => {
        const options = JSON.parse(challenge_json);
        const b64ToBuf = (b64url) => {
            const standardB64 = b64url
                .replace(/-/g, '+')
                .replace(/_/g, '/')
                .padEnd(b64url.length + (4 - b64url.length % 4) % 4, '=');

            return Uint8Array.from(atob(standardB64), c => c.charCodeAt(0)).buffer;
        };
        options.publicKey.challenge = b64ToBuf(options.publicKey.challenge);
        options.publicKey.user.id = b64ToBuf(options.publicKey.user.id);
        if (options.publicKey.excludeCredentials) {
            options.publicKey.excludeCredentials = options.publicKey.excludeCredentials.map(cred => ({
                ...cred,
                id: b64ToBuf(cred.id)
            }));
        }
        try {
            console.log("Requesting WebAuthn Prompt with options:", options);
            const credential = await navigator.credentials.create(options);
            const bufToB64 = (buf) => btoa(String.fromCharCode(...new Uint8Array(buf)));
            resolve(JSON.stringify({
                id: credential.id,
                rawId: btoa(String.fromCharCode(...new Uint8Array(credential.rawId))),
                response: {
                    attestationObject: bufToB64(credential.response.attestationObject),
                    clientDataJSON: bufToB64(credential.response.clientDataJSON),
                },
                type: credential.type,
            }));
        } catch (err) {
            console.error("WebAuthn Error:", err.name, err.message);
            reject(err.name + ": " + err.message);
        }
    });
}
