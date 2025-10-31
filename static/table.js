/* webrust/static/table.js */
window.webrustInitTable = function(tableId, filter, pageSize, hasPagination) {
    const table = document.getElementById(tableId);
    if (!table) return;
    const tbody = table.querySelector('tbody');
    const allRows = Array.from(tbody.querySelectorAll('tr'));
    const headers = Array.from(table.querySelectorAll('.sort-header'));
    const filterInputs = filter ? Array.from(table.querySelectorAll('.filter-input')) : [];
    const wrapper = table.closest('.table-container');
    const paginationDiv = wrapper?.querySelector('.pagination-controls');
    const hasRowHeader = (allRows[0]?.firstElementChild?.tagName || '') === 'TH';
    const offset = hasRowHeader ? 1 : 0;
    const collator = new Intl.Collator(undefined, { numeric: false, sensitivity: 'base' });
    const headerTypes = headers.map(h => h.getAttribute('data-type') || 'string');

    if (!filter && headers.length === 0 && !hasPagination) return;

    let filteredRows = allRows.slice();
    let currentPage = 0;
    let sortCol = -1;
    let sortDir = 1;
    let totalPages = 1;
    let lastSortCol = -2;
    let keyCache = null;

    const filterCache = new Array(filterInputs.length).fill('');
    const textCache = new Map();

    function getCellText(row, i) {
        let arr = textCache.get(row);
        if (!arr) {
            arr = Array.from(row.cells).slice(offset).map(td => td.textContent || '');
            textCache.set(row, arr);
        }
        return arr[i] || '';
    }

    function parseKey(value, type) {
        if (type === 'number') { const x = parseFloat(value); return Number.isNaN(x) ? -Infinity : x; }
        if (type === 'date') { const x = Date.parse(value); return Number.isNaN(x) ? -8640000000000000 : x; }
        return value.toLowerCase();
    }

    function applyFilters() {
        if (!filter || filterInputs.length === 0) { filteredRows = allRows.slice(); return; }
        if (!filterCache.some(Boolean)) { filteredRows = allRows.slice(); return; }
        filteredRows = allRows.filter(row => {
            for (let i = 0; i < filterCache.length; i++) {
                const term = filterCache[i];
                if (term && getCellText(row, i).toLowerCase().indexOf(term) === -1) return false;
            }
            return true;
        });
    }

    function applySort() {
        if (sortCol < 0) return;
        if (sortCol !== lastSortCol) { keyCache = new Map(); lastSortCol = sortCol; }
        const type = headerTypes[sortCol];
        const isString = type === 'string';
        const keyOf = row => {
            let k = keyCache.get(row);
            if (k === undefined) { k = parseKey(getCellText(row, sortCol).trim(), type); keyCache.set(row, k); }
            return k;
        };
        filteredRows.sort((a, b) => {
            const ka = keyOf(a), kb = keyOf(b);
            return isString ? sortDir * collator.compare(ka, kb) : sortDir * (ka < kb ? -1 : ka > kb ? 1 : 0);
        });
    }

    function render() {
        const perPage = hasPagination ? Math.max(1, pageSize | 0) : Math.max(1, filteredRows.length);
        totalPages = Math.max(1, Math.ceil(filteredRows.length / perPage));
        if (currentPage >= totalPages) currentPage = totalPages - 1;
        const start = currentPage * perPage;
        const end = start + perPage;
        const frag = document.createDocumentFragment();
        tbody.textContent = '';
        for (let i = start; i < end && i < filteredRows.length; i++) frag.appendChild(filteredRows[i]);
        tbody.appendChild(frag);
        if (paginationDiv && hasPagination) {
            let html = '';
            for (let i = 0; i < totalPages; i++) html += `<button class="${i === currentPage ? 'active' : ''}" data-page="${i}">${i + 1}</button>`;
            paginationDiv.innerHTML = html;
        }
    }

    function update(resetPage = true) {
        applyFilters();
        applySort();
        if (resetPage) currentPage = 0;
        render();
    }

    let debounceId = 0;
    if (filter) {
        filterInputs.forEach((input, idx) => {
            input.addEventListener('input', function() {
                filterCache[idx] = this.value.toLowerCase();
                clearTimeout(debounceId);
                debounceId = setTimeout(() => update(true), 120);
            });
        });
    }

    headers.forEach((header, idx) => {
        header.addEventListener('click', function() {
            sortDir = (sortCol === idx) ? -sortDir : 1;
            sortCol = idx;
            headers.forEach(h => h.classList.remove('sort-asc', 'sort-desc'));
            this.classList.add(sortDir === 1 ? 'sort-asc' : 'sort-desc');
            update(true);
        });
    });

    if (paginationDiv) {
        paginationDiv.addEventListener('click', e => {
            const btn = e.target.closest('button');
            if (!btn) return;
            const n = parseInt(btn.dataset.page, 10);
            if (Number.isInteger(n) && n !== currentPage) { currentPage = n; render(); }
        });
    }

    update(true);
};