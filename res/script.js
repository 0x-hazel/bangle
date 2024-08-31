function add() {
    let form = document.forms['add'];
    let frame = document.getElementById('dummy');
    frame.onload = () => {
        let result = JSON.parse(frame.contentDocument.body.innerText);
        if (result.success) {
            location.replace(location.href)
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