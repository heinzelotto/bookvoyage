import { user, token, isAuthenticated } from '../stores.js';

export function getAuthHeader() {
    let currentToken;
    token.subscribe((value) => (currentToken = value))();
    console.log("token to be used: " + JSON.stringify(currentToken));
    return currentToken ? `Bearer ${currentToken.token}` : '';
}


// // export const user = writable(JSON.parse(localStorage.getItem('user')));
// // export const isAuthenticated = writable(!!localStorage.getItem('token'));

export async function login(username, password) {
    const response = await fetch('/api/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, password })
    });

    if (response.ok) {
        const data = await response.json();
        token.set({ token: data.token });
        user.set({ username: username });
        isAuthenticated.set(true);
    } else {
        throw new Error('Login failed');
    }
}

export function logout() {
    user.set({});
    token.set({});
    isAuthenticated.set(false);
}

// export async function login(username, password) {
//     const response = await fetch('/api/login', {
//         method: 'POST',
//         headers: { 'Content-Type': 'application/json' },
//         body: JSON.stringify({ username, password })
//     });

//     if (response.ok) {
//         const data = await response.json();
//         $token = { token: data.token };
//         $user = { username: JSON.stringify({ username }) };
//         $isAuthenticated = true;
//     } else {
//         throw new Error('Invalid credentials');
//     }
// }

// export function logout() {
//     $user = {};
//     $token = {};
//     $isAuthenticated = false;
// }