import "./libs/shim/core.js";
import "./libs/shim/urijs.js";
import { group } from "k6";



export let options = {
    maxRedirects: 4,
    duration: "3m",
    vus: 100
};



const Request = Symbol.for("request");

// noinspection JSUnresolvedVariable
postman[Symbol.for("initial")]({
  options,
  collection: {
    BASE_URL: `${__ENV.BASE_URL}`,
    LAST_USER_ID: `${__ENV.LAST_USER_ID || 1}`,
    REQUESTS_HOST: `${__ENV.REQUESTS_HOST || "arch.homework"}`,
  }
});


export default function() {
  group("users", function() {
    postman[Request]({
      name: "03_USERS_GET_BY_ID",
      id: "c6d38060-de47-4575-9077-d090ae7797ee",
      method: "GET",
      address: "{{BASE_URL}}/users/a/{{LAST_USER_ID}}/",
      headers: {
        Host: "{{REQUESTS_HOST}}"
      },
      post(response) {
        pm.test("Status test", function() {
          pm.response.to.have.status(200);
        });
      }
    });
  });
}
