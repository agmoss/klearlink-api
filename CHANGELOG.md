# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2025-04-06

### 🚀 Features

- Adding in consumer_information_indicator

### 📚 Documentation

- Add security section detailing API security features and compliance
- Update API docs

### ⚙️ Miscellaneous Tasks

- Better markdown
- Fix action
- Adding in .idea integration

## [0.1.0] - 2025-03-06

### 🚀 Features

- Init
- Implement API key and username authentication for endpoints
- Working auth
- Implement update_consumer_credit method to update records in DB
- Dto implementation
- Working update
- Better impl
- Implement view_consumer_credit method to retrieve consumer credit record
- View
- Format
- Update all fields in update_consumer_credit method in main.rs
- Add tenant column to consumer credit and populate during submission
- New consumer credit 2 temp
- Better types and error handling
- Adding in error handlers
- Working serde error
- Clean up
- Use custom responder for authentication failure in auth module
- Adding in auth failure json
- Proper duplicate handling
- Reorg
- Reorg
- Cleaning up
- Better error message
- Implement view_consumer_match route to check consumer match status
- Implement view_consumer_match to retrieve and return matches by ID
- Add matched_on field to ConsumerCreditDto in response records
- Add ConsumerMatchDto and update view_consumer_match route
- Working on matches
- Matching working
- Exclude current tenant matches in view_consumer_match route
- Fixing tests
- Sql linter
- Better makefile
- Adding in users
- Implement user validation using database query in AuthStore
- Add id field to ApiKeyAuth and include it in Outcome::Success
- Add method to AuthStore to insert dummy user if none exist
- Initial user implementation
- Working on dto validation
- Adding json error responses
- Adding in pool stuff
- Working pool implementation
- Working pool implementation
- Service implementation
- Adding in impl service
- Change api_key type from String to UUID in Users struct
- Working on uuid
- Working uuid impl
- Tests
- Implement CRUD endpoints and service for users entity
- Starting on users
- Working on users functionality
- Adding in better dto validation
- Improve create_user_service method with better error handling and feedback
- Better names
- Add role field to users model for lender and admin roles
- Add setup function to create test users before running tests
- Working on roles
- Add endpoint to delete a user by their username
- Working on tests
- Add route and service method to delete consumer credits by username
- Adding in sort of better tests
- Cleaning up
- Restrict user routes access to admin role only
- Db operation handler
- Refactor
- Add database indexes for improved query performance
- Adding in more tests
- Updating deps
- Add email validation to ConsumerFactsDto for RFC standards
- Adding in dto validation
- Adding in more validators
- Touching up readme
- Sync readme
- Adding in new dto
- Update view_consumer_match to return ConsumerMatchDtoAlt format
- Fixing dto
- Better name
- Adding in optional date fields
- Add SQL statement to insert test admin user in seed migration
- Working on date fields
- Adding in data fields to the dto
- Adding in cors
- Add method to convert ConsumerCreditModel and matches to ConsumerMatchDto
- Moving more to the dto layer
- Adding in some doc comments
- Add base route to return API version, description, and help link
- Better auth dto
- Better tests
- Better tests
- Omit null fields from ConsumerCreditDto in From implementation
- Return custom error message for duplicate consumer credit record
- Update consumer credit model to use fields from self if present
- Add update DTOs for consumer and credit facts
- Strong update
- Add handling for CheckViolation in error responses
- Return JSON with constraint name in CheckViolation error response
- Add validation to ensure all or none of credit facts fields are supplied
- Better validation
- Adding in another validation
- Implement event sourcing for consumer_credit entity changes
- Baseline
- Working tracking
- Cleaning up
- Better names
- Add Dockerfile and docker-compose.yml for Rust application setup
- Adding in docker stuff
- Adding in logger
- Inclusions
- Optimized dockerfile
- Adding in production and staging workflows
- Optimized docker setup
- Adding in image name
- Package upgrade and adding in env to base route
- Adding in better linting and file check
- Adding in changelog

### 🐛 Bug Fixes

- Clone fields in convert_record_to_consumer_credit for ownership issues
- Update parameter name in view_consumer_credit function for clarity
- Improve error handling in view_consumer_credit method
- Remove unnecessary QueryableByName derive from ConsumerCredit struct
- Update import statement for InsertConsumerCredit in tests module
- Correct conversion of ConsumerCredit to ConsumerCreditDto in routes.rs
- Remove duplicate use statement and define get_user function correctly
- Pass Db connection pool to get_user function in ApiKeyAuth implementation
- Use custom responder for errors in from_request method
- Include username in 'user not found' error message
- Remove duplicate InsertUserModel definition and include role in to_insert_user method
- Restrict admin role check to user routes only
- Ensure proper error mapping in get_user_id_by_username function
- Correct institution_names description in README.md
- Correct institution_names assignment in From<ConsumerCreditModel> impl
- Remove test admin user from down migration SQL file
- Specify type for records field in ConsumerCreditService closure
- Update parameter type in update_consumer_credit method
- Remove env copy

### 🚜 Refactor

- Optimize consumer credit methods by reducing code duplication
- Remove unused import of ConsumerCreditRecord in main.rs
- Move convert_record_to_consumer_credit to ConsumerCreditRecord impl
- Consolidate ConsumerFactsDto and CreditFactsDto into ConsumerCreditRecord
- Restructure ConsumerCreditRecord to use nested DTOs for facts
- Consolidate user database calls into a single function
- Use connection pool in consumer_credit routes for DB operations
- Update AuthStore to use database pool for connection management
- Move route handlers to service implementation for better separation
- Extract admin role check into reusable utility function
- Rename create_user_service to create_user in UserService
- Simplify user service methods by introducing a database operation handler
- Introduce helper function for database operations in consumer credit service
- Optimize async execution in consumer credit and user services
- Consolidate error handling into a single conversion method
- Remove null fields from ConsumerCreditDto conversion logic
- Improve error handling and type alias for DieselError
- Simplify field access in to_update_consumer_credit_model method
- Simplify event logging in ConsumerCreditService methods

### 📚 Documentation

- Expanding readme
- Enhance README.md with Table of Contents and additional sections
- Update Data Standards with field length and date validation rules
- Update README.md to include additional validations for fields
- Add validations for first and last name to Data Standards section

### 🎨 Styling

- Format function parameters for better readability
- Format code by adding newlines for readability

### 🧪 Testing

- Update test payloads to match new ConsumerCreditRecord structure
- Add more positive and negative test cases for consumer credit API
- Add unit test for view_consumer_match endpoint with records
- Update test to add records via API instead of direct database insert
- Add asynchronous tests for users and consumer_credit routes
- Add comprehensive test suite for user and consumer credit endpoints
- Seperate build and push steps
- Remove version

### ⚙️ Miscellaneous Tasks

- Remove unused migrations
- Better names
- Add serde_json import to routes for JSON handling
- Slight cleanup
- Adding in user testing
- Working on it
- Add initial SQL migration file for user seeding
- Working on update
- Import over qualify
- Rename
- Rename
- Dto simple
- Remove imports
- Adding in seperate build and push step
- Consolidating routes
- Update env example
- Release 0.1.0

<!-- generated by git-cliff -->
