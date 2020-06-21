import * as React from 'react';
import * as ReactDOM from 'react-dom';

import { Router } from '@reach/router';
import { Main } from './page/main/Main';

function Tsurezure() {
    return <div className="main">
        <h1 id="title">徒然</h1>
        <Router>
            <Main path="/" />
        </Router>
    </div>;
}

ReactDOM.render(
    <Tsurezure />,
    document.getElementById('main')
);
