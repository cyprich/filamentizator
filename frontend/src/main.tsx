import {StrictMode} from 'react'
import {createRoot} from 'react-dom/client'
import './index.css'
import {BrowserRouter, Route, Routes} from "react-router";
import Home from "./pages/Home.tsx";
import Filaments from "./pages/Filaments.tsx";
import Account from "./pages/Account.tsx";
import Navbar from "./components/Navbar.tsx";



createRoot(document.getElementById('root')!).render(
    <StrictMode>
        <BrowserRouter>
            <Navbar/>
            <Routes>
                <Route path={"/"} element={ <Home/> }/>
                <Route path={"/filaments"} element={ <Filaments/> }/>
                <Route path={"/account"} element={ <Account/> }/>
            </Routes>
        </BrowserRouter>
    </StrictMode>,
)
