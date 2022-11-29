class State {
  constructor() {
    this.user = null;
  }

  /*
    "id": string
    "name": string
    "avatar": string
    "new": bool
  */
  async getUser() {
    if (getCookie("session") == undefined) return null;
    if (this.user != null) return this.user;
    let data = await (await fetch("/api/self_info")).json();
    this.user = data;
    return data;
  }
}

// From https://javascript.info/cookie
function getCookie(name) {
  let matches = document.cookie.match(
    new RegExp(
      "(?:^|; )" +
        name.replace(/([\.$?*|{}\(\)\[\]\\\/\+^])/g, "\\$1") +
        "=([^;]*)"
    )
  );
  return matches ? decodeURIComponent(matches[1]) : undefined;
}

export let state = new State();
