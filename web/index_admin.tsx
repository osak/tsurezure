import * as React from 'react';
import * as ReactDOM from 'react-dom';

import { Router } from '@reach/router';
import { Edit } from './page/admin/Edit';

function TsurezureAdmin() {
    return <div className="main">
        <h1 id="title">徒然Admin</h1>
        <Router>
            <Edit path="/admin/posts/:id" />
        </Router>
    </div>;
}

ReactDOM.render(
    <TsurezureAdmin />,
    document.getElementById('main')
);
