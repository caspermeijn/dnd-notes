/* Copyright (C) 2022 Casper Meijn <casper@meijn.net>
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::logs;
use crate::logs::LogEntry;
use crate::AdditionalNames;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(utoipa::OpenApi)]
#[openapi(
    handlers(
        crate::hello,
        crate::hello_name,
        logs::get_logs,
    ),
    components(
        AdditionalNames,
        LogEntry,
    ),
    tags(
        (name = "logs", description = "Log management endpoints.")
    ),
)]
struct ApiDoc;

pub fn config(cfg: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();

    cfg.service(SwaggerUi::new("/api-doc/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi));
}
