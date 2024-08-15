function submitForm(id) {
    document.forms[id].submit()
    setTimeout(() => {location.reload()}, 500)
}