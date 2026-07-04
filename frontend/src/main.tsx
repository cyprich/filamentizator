import {StrictMode} from 'react'
import {createRoot} from 'react-dom/client'
import './index.css'
import {BrowserRouter, Route, Routes} from "react-router";
import Home from "./pages/Home.tsx";
import Filaments from "./pages/Filaments.tsx";
import Account from "./pages/Account.tsx";
import Navbar from "./components/Navbar.tsx";
import Footer from "./components/Footer.tsx";
import Device from "./pages/Device.tsx";
import PageNotFound from "./pages/PageNotFound.tsx";
import Labels from "./pages/Labels.tsx";
import { Toast } from "@heroui/react";
import Filament from "./pages/Filament.tsx";

export const BASE_URL = "http://localhost:5000/api/v3"

createRoot(document.getElementById('root')!).render(
    <StrictMode>
        <Toast.Provider/>
        <BrowserRouter>
            <Navbar/>
            <Routes>
                <Route path={"/"} element={ <Home/> }/>
                <Route path={"/filaments"} element={ <Filaments/> }/>
                <Route path={"/filaments/:id"} element={ <Filament/> }/>
                <Route path={"/labels"} element={ <Labels/> }/>
                <Route path={"/device"} element={ <Device/> }/>
                <Route path={"/account"} element={ <Account/> }/>
                <Route path={"*"} element={ <PageNotFound/> } />
            </Routes>
            <Footer/>
        </BrowserRouter>
    </StrictMode>
)
