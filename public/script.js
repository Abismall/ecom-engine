const API_URL = 'http://127.0.0.1:8000';
const cache = {};

document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('addProductForm').addEventListener('submit', async (event) => {
        event.preventDefault();

        const name = document.getElementById('productName').value;
        const size = document.getElementById('size').value;
        const color = document.getElementById('color').value;
        const weight = document.getElementById('weight').value;
        const weight_unit = document.getElementById('weightUnit').value;
        const width = document.getElementById('width').value;
        const height = document.getElementById('height').value;
        const category_id = document.getElementById('categoryId').value;
        const brand_id = document.getElementById('brandId').value;
        const price = document.getElementById('price').value;
        const tax_rate = document.getElementById('taxRate').value;

        const product = {
            name: name || null,
            in_stock: false,
            size: size || null,
            color: color || null,
            weight: weight ? parseFloat(weight) : 0,
            weight_unit: weight_unit || null,
            width: width ? parseFloat(width) : 0,
            height: height ? parseFloat(height) : 0,
            category_id: category_id ? parseInt(category_id) : null,
            brand_id: brand_id ? parseInt(brand_id) : null,
            price: price ? parseFloat(price) : 0,
            tax_rate: tax_rate ? parseFloat(tax_rate) : 0
        };

        await addProduct(product);
        invalidateCache('products');
        fetchProducts();
    });
    document.getElementById('pauseProcessorButton').addEventListener('click', async () => {
        await pauseProcessor();
        fetchProcessorStats();
    });

    document.getElementById('startProcessorButton').addEventListener('click', async () => {
        await startProcessor();
        fetchProcessorStats();
    });
    document.getElementById('addBrandForm').addEventListener('submit', async (event) => {
        event.preventDefault();
        const name = document.getElementById('brandName').value;
        await addBrand({ name });
        invalidateCache('brands');
        fetchBrands();
    });

    document.getElementById('addOrderLineForm').addEventListener('submit', async (event) => {
        event.preventDefault();
        const orderLine = {
            cart_id: parseInt(document.getElementById('cartId').value),
            product_id: parseInt(document.getElementById('productId').value),
            quantity: parseInt(document.getElementById('quantity').value)
        };
        await addOrderLine(orderLine);
        fetchCarts();
    });

    document.getElementById('removeOrderLineForm').addEventListener('submit', async (event) => {
        event.preventDefault();
        const id = parseInt(document.getElementById('orderLineId').value);
        await removeOrderLine(id);
        fetchCarts();
    });

    document.getElementById('createCartForm').addEventListener('submit', async (event) => {
        event.preventDefault();
        await createCart();
        fetchCarts();
    });

    document.getElementById('loginForm').addEventListener('submit', async (event) => {
        event.preventDefault();
        const credentials = {
            username: document.getElementById('username').value,
            password: document.getElementById('password').value
        };
        await login(credentials);
    });

    document.getElementById('addDiscountForm').addEventListener('submit', async (event) => {
        event.preventDefault();
        const discount = {
            name: document.getElementById('discountName').value,
            discount_type: document.getElementById('discountType').value,
            value: parseFloat(document.getElementById('discountValue').value),
            start_date: document.getElementById('startDate').value,
            end_date: document.getElementById('endDate').value,
            min_quantity: parseInt(document.getElementById('minQuantity').value)
        };
        await addDiscount(discount);
        invalidateCache('discounts');
        fetchDiscounts();
    });

    document.getElementById('associateDiscountForm').addEventListener('submit', async (event) => {
        event.preventDefault();
        const discountId = parseInt(document.getElementById('discountId').value);
        const associationType = document.getElementById('associationType').value;
        const associationId = parseInt(document.getElementById('associationId').value);

        if (associationType === 'category') {
            await addDiscountCategory({ discount_id: discountId, category_id: associationId });
        } else if (associationType === 'brand') {
            await addDiscountBrand({ discount_id: discountId, brand_id: associationId });
        } else if (associationType === 'product') {
            await addDiscountProduct({ discount_id: discountId, product_id: associationId });
        }

        invalidateCache('discounts');
        fetchDiscounts();
    });

    // Add event listeners for navigation
    const navLinks = document.querySelectorAll('nav ul li a');
    navLinks.forEach(link => {
        link.addEventListener('click', (event) => {
            event.preventDefault();
            const view = link.getAttribute('data-view');
            getViewData(view);
            showView(view);
        });
    });

    if (!localStorage.getItem('token')) showView('login');
    else {
        getViewData('products');
        showView('products');
    }
});

async function addOrderLine(orderLine) {
    await fetch(`${API_URL}/orderline`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(orderLine),
    });
}

async function removeOrderLine(id) {
    await fetch(`${API_URL}/orderline/${id}`, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ id }),
    });
}

async function createCart() {
    await fetch(`${API_URL}/cart`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        }
    });
}
async function pauseProcessor() {
    await fetch(`http://127.0.0.1:3030/pause`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        }
    });
}

async function startProcessor() {
    await fetch(`http://127.0.0.1:3030/start`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        }
    });
}



async function fetchCarts() {
    if (cache['carts'] && cache['carts'].html) {
        displayCachedHTML('cartList', cache['carts'].html);
    } else {
        const response = await fetch(`${API_URL}/cart`, { method: "GET" });
        const carts = await response.json();
        const html = generateCartsHTML(carts);
        cache['carts'] = { data: carts, html };
        displayCachedHTML('cartList', html);
    }
}

function generateCartsHTML(carts) {
    return carts.map(item => `
        <div class="list-item">
            <div class="details">Cart ID: ${item.cart.id}</div>
            <div class="details">${JSON.stringify(item.order_lines)}</div>
          
            <button data-id="${item.cart.id}" class="delete">Empty Cart</button>
        </div>
    `).join('');
}

function displayCachedHTML(elementId, html) {
    const element = document.getElementById(elementId);
    element.innerHTML = html;
    addDeleteEventListeners(elementId);

}

function showView(view) {
    const views = document.querySelectorAll('.view');
    views.forEach(v => {
        v.classList.remove('active');
    });

    document.getElementById(`${view}View`).classList.add('active');
}

async function getViewData(view) {
    switch (view) {
        case "products":
            fetchProducts();
            break;
        case "brands":
            fetchBrands();
            break;
        case "carts":
            fetchCarts();
            break;
        case "discounts":
            fetchDiscounts();
            break;
        case "updateProcessor":
            break;
        default:
            break;
    }
}


function invalidateCache(type) {
    delete cache[type];
}

async function login(credentials) {
    const response = await fetch(`${API_URL}/login`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(credentials),
    });
    if (response.ok) {
        const data = await response.json();
        localStorage.setItem('token', data.token);
        getViewData('products');
        showView('products');
    } else {
        alert('Login failed!');
    }
}

async function fetchProducts() {
    if (cache['products'] && cache['products'].html) {
        displayCachedHTML('productList', cache['products'].html);
    } else {
        const response = await fetch(`${API_URL}/product`);
        const products = await response.json();
        const html = generateProductsHTML(products);
        cache['products'] = { data: products, html };
        displayCachedHTML('productList', html);
    }
}

function generateProductsHTML(products) {
    return products.map(item => `
        <div class="list-item">
            <div class="details">Product: ${item.product.name}</div>
            <div class="details">Brand ID: ${item.product.brand_id}</div>
            <div class="details">Category ID: ${item.product.category_id}</div>
            <div class="details">Price: $${item.product.price.toFixed(2)}</div>
            <div class="details">In stock: ${item.product.in_stock ? 'Yes' : 'No'}</div>
            <button data-id="${item.product.id}" class="delete">Delete</button>
        </div>
    `).join('');
}

async function addProduct(product) {
    await fetch(`${API_URL}/product`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(product),
    });
}

async function deleteProduct(id) {
    await fetch(`${API_URL}/product/id/${id}`, {
        method: 'DELETE',
    });
}

async function fetchBrands() {
    if (cache['brands'] && cache['brands'].html) {
        displayCachedHTML('brandList', cache['brands'].html);
    } else {
        const response = await fetch(`${API_URL}/brand`);
        const brands = await response.json();
        const html = generateBrandsHTML(brands);
        cache['brands'] = { data: brands, html };
        displayCachedHTML('brandList', html);
    }
}

function generateBrandsHTML(brands) {
    return brands.map(brand => `
        <div class="list-item">
            <div class="details">Brand: ${brand.name}</div>
            <button data-id="${brand.id}" class="delete">Delete Brand</button>
        </div>
    `).join('');
}
async function addBrand(brand) {
    await fetch(`${API_URL}/brand`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(brand),
    });
}

async function deleteBrand(id) {
    await fetch(`${API_URL}/brand/id/${id}`, {
        method: 'DELETE',
    });
}

async function fetchDiscounts() {
    if (cache['discounts'] && cache['discounts'].html) {
        displayCachedHTML('discountList', cache['discounts'].html);
    } else {
        const response = await fetch(`${API_URL}/discount`);
        const discounts = await response.json();
        const html = generateDiscountsHTML(discounts);
        cache['discounts'] = { data: discounts, html };
        displayCachedHTML('discountList', html);
    }
}

function generateDiscountsHTML(discounts) {
    return discounts.map(discount => `
        <div class="list-item">
            <div class="details">Discount Name: ${discount.name}</div>
            <div class="details">Type: ${discount.discount_type}</div>
            <div class="details">Value: ${discount.value}</div>
            <button data-id="${discount.id}" class="delete">Delete Discount</button>
        </div>
    `).join('');
}

async function addDiscount(discount) {
    await fetch(`${API_URL}/discount`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            ...discount,
            start_date: discount.start_date + ":00",
            end_date: discount.end_date + ":00"
        }),
    });
}

async function deleteDiscount(id) {
    await fetch(`${API_URL}/discount/${parseInt(id)}`, {
        method: 'DELETE',
    });
}

async function addDiscountCategory(discountCategory) {
    await fetch(`${API_URL}/discount/category`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(discountCategory),
    });
}

async function addDiscountBrand(discountBrand) {
    await fetch(`${API_URL}/discount/brand`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(discountBrand),
    });
}

async function addDiscountProduct(discountProduct) {
    await fetch(`${API_URL}/discount/product`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(discountProduct),
    });
}
async function addDeleteEventListeners(elementId) {
    document.querySelectorAll(`#${elementId} .delete`).forEach(button => {
        button.addEventListener('click', async (event) => {
            const id = event.target.getAttribute('data-id');
            await deleteDiscount(id);
            invalidateCache('products');
            fetchProducts();
        });
    });
}