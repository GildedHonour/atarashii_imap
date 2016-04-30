/**
 * Copyright (c) 2016 Alex Maslakov, <http://gildedhonour.com>, <http://alexmaslakov.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * For questions and comments about this product, please see the project page at:
 *
 * https://github.com/GildedHonour/atarashii_imap
 *
 */

//IMAP Errors

use std::error;
use std::fmt;

#[derive(Debug)]
pub struct AppendError;

impl error::Error for AppendError {
  fn description(&self) -> &str {
    unimplemented!()
  }

  fn cause(&self) -> Option<&error::Error> {
    unimplemented!()
  }
}

impl fmt::Display for AppendError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "IO error: {}", "test")
  }
}

#[derive(Debug)]
pub struct SearchError;

#[derive(Debug)]
pub struct FetchError;

#[derive(Debug)]
pub struct StoreError;

#[derive(Debug)]
pub struct CopyError;

#[derive(Debug)]
pub struct UnknownCommandOrInvalidArgsError;

#[derive(Debug)]
pub struct NoSuchMailboxError;

#[derive(Debug)]
pub struct InvalidCredentialsError;

#[derive(Debug)]
pub struct UnableToCreateMailboxError;

#[derive(Debug)]
pub struct UnableToDeleteMailboxError;

#[derive(Debug)]
pub struct UnableToRenameMailboxError;

#[derive(Debug)]
pub struct UnableToSubscribeToMailboxError;

#[derive(Debug)]
pub struct LoginError;