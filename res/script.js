const template = document.getElementById('template');

function add() {
    let form = document.forms['add'];
    let frame = document.getElementById('dummy');
    frame.onload = () => {
        let result = JSON.parse(frame.contentDocument.body.innerText);
        if (result.success) {
            let entry = template.innerHTML
            entry = entry.replace(/%name%/g, result.details.name)
            entry = entry.replace(/%id%/g, result.details.id)
            entry = entry.replace(/%url%/g, result.details.url)
            entry = entry.replace(/%key%/g, result.details.key)
            document.getElementById('bangs-list').innerHTML += entry
            form.querySelector('.input-bang').value = ""
            form.querySelector('.input-url').value = ""
        } else {
            // show some kind of error message?
        }
    }
    form.submit();
}

function save(id) {
    let form = document.forms[id];
    let frame = document.getElementById('dummy');
    frame.onload = () => {
        let result = JSON.parse(frame.contentDocument.body.innerText);
        if (result.success) {
            form.querySelector('.bang-url').innerText = result.url;
            cancel(id);
        } else {
            cancel(id);
            // show some kind of error message?
        }
    }
    form.submit();
}

function del(id) {
    let form = document.forms[id];
    let frame = document.getElementById('dummy');
    frame.onload = () => {
        let result = JSON.parse(frame.contentDocument.body.innerText);
        if (result.success) {
            form.parentElement.remove();
        } else {
            // show some kind of error message?
        }
    }
    form.submit();
}

function edit(id) {
    let form = document.forms[id];
    let url_entry = form.querySelector('.bang-url-entry');
    form.querySelector('.bang-url').style.display = 'none';
    url_entry.setAttribute('type', 'text');
    form.querySelector('.btns-sub').style.display = 'inline';
    form.querySelector('.btns-main').style.display = 'none';
    form.setAttribute('action', '/edit');
    form.setAttribute('onsubmit', `save('${id}')`);
    url_entry.focus();
    url_entry.selectionStart = url_entry.selectionEnd = url_entry.value.length;
}

function cancel(id) {
    let form = document.forms[id];
    let prev_url = form.querySelector('.bang-url');
    let url_entry = form.querySelector('.bang-url-entry');
    prev_url.style.display = 'inline';
    url_entry.setAttribute('type', 'hidden');
    url_entry.setAttribute('value', prev_url.innerText);
    form.querySelector('.btns-sub').style.display = 'none';
    form.querySelector('.btns-main').style.display = 'inline';
    form.setAttribute('action', '/del');
    form.removeAttribute('onsubmit')
}

function setFallback(id) {
    let form = document.forms[id];
    let frame = document.getElementById('dummy');
    frame.onload = () => {
        let result = JSON.parse(frame.contentDocument.body.innerText);
        if (result.success) {
            form.querySelector('.fallback').innerText = "Currently: " + result.current;
        } else {
            // show some kind of error message?
        }
    }
    form.submit();
}