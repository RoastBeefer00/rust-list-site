use maud::{html, Markup, Render, DOCTYPE};

use super::list_group::ListGroup;

pub struct IndexTemplate {
    pub groups: Vec<ListGroup>,
}

impl Render for IndexTemplate {
    fn render(&self) -> Markup {
        html! {
            (DOCTYPE)
            html class="min-h-screen bg-gray-900 text-white" {
                head {
                    script src="https://cdn.tailwindcss.com" {}
                    script src="https://unpkg.com/htmx.org@2.0.4" integrity="sha384-HGfztofotfshcF7+8n44JQL2oJmowVChPTg48S+jvZoztPfvwD79OC/LTtG6dMp+" crossorigin="anonymous" {}
                    script type="module" {
                        "import { initializeApp } from 'https://www.gstatic.com/firebasejs/11.1.0/firebase-app.js';
                import { GoogleAuthProvider, getAuth, signInWithRedirect, getRedirectResult, setPersistence } from 'https://www.gstatic.com/firebasejs/11.1.0/firebase-auth.js';

                const provider = new GoogleAuthProvider();

                const firebaseConfig = {
                    apiKey: 'AIzaSyAwYYAbVrDERwdpRp4kOU8fh5lW6fqFt0s',
                    authDomain: 'r-j-magenta-carrot-42069.firebaseapp.com',
                    projectId: 'r-j-magenta-carrot-42069',
                };

                let auth_token = null;
                const app = initializeApp(firebaseConfig);
                let auth = getAuth(app);

                auth.onAuthStateChanged(async user => {
                    if (!user) {
                        signInWithRedirect(auth, provider);
                    } else {
                        let token = await user.getIdToken(true);
                        auth_token = token;
                        htmx.ajax('GET', '/groups', {
                            target: '#mylists',
                            swap: 'outerHTML'
                        });
                    }
                });

                // gate htmx requests on the auth token
                htmx.on('htmx:confirm', (e)=> {
                    // if there is no auth token
                    if(auth_token == null) {
                        // stop the regular request from being issued
                        e.preventDefault()
                        // only issue it once the auth promise has resolved
                        // auth.then(() => e.detail.issueRequest())
                    }
                })

                // add the auth token to the request as a header
                htmx.on('htmx:configRequest', (e)=> {
                    e.detail.headers['AUTHORIZATION'] = 'Bearer ' + auth_token
                })"
                    }
                    title id="title" { "Hey, Listen!" }
                }
                body id="body" {
                    div id="mylists" {}
                }
            }
        }
    }
}
