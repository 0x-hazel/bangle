// Lucide ISC License, available at https://lucide.dev/license
const icons = {
    edit: '<path d="M12 20h9"/><path d="M16.376 3.622a1 1 0 0 1 3.002 3.002L7.368 18.635a2 2 0 0 1-.855.506l-2.872.838a.5.5 0 0 1-.62-.62l.838-2.872a2 2 0 0 1 .506-.854z"/><path d="m15 5 3 3"/>',
    delete: '<path d="M3 6h18"/><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"/><path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"/><line x1="10" x2="10" y1="11" y2="17"/><line x1="14" x2="14" y1="11" y2="17"/>',
    okay: '<path d="M20 6 9 17l-5-5"/>',
    cancel: '<path d="M18 6 6 18"/><path d="m6 6 12 12"/>'
}

function submitForm(id) {
    document.forms[id].submit()
    setTimeout(() => {location.reload()}, 500)
}