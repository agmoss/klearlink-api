# KlearLink API

- [KlearLink API](#klearlink-api)
  - [**Authentication Guide**](#authentication-guide)
    - [**Authentication Format**](#authentication-format)
    - [**Error Responses**](#error-responses)
  - [Error Handling](#error-handling)
    - [Error Responses](#error-responses-1)
  - [1. Submit a consumer credit record](#1-submit-a-consumer-credit-record)
  - [2. Update a consumer credit record](#2-update-a-consumer-credit-record)
  - [3. View a submitted consumer credit record](#3-view-a-submitted-consumer-credit-record)
  - [4. View Consumer Match](#4-view-consumer-match)
    - [matched\_on](#matched_on)
  - [Security](#security)
    - [Authentication](#authentication)
    - [Monitoring and Logging](#monitoring-and-logging)
    - [Compliance and Protection](#compliance-and-protection)
  - [Appendix](#appendix)
    - [A. Definitions](#a-definitions)
    - [B. Consumer Information Indicator](#b-consumer-information-indicator)
    - [B. Data Standards](#b-data-standards)
      - [1. E.164 Phone Number Validation](#1-e164-phone-number-validation)
        - [**Regex Pattern:**](#regex-pattern)
        - [**Description:**](#description)
        - [**Rules:**](#rules)
        - [**Examples:**](#examples)
      - [2. CAN/CSA-Z109.1-01 Canadian Address Validation](#2-cancsa-z1091-01-canadian-address-validation)
        - [**Regex Pattern:**](#regex-pattern-1)
        - [**Description:**](#description-1)
        - [**Rules:**](#rules-1)
        - [**Examples:**](#examples-1)
      - [3. RFC 5322/822 Email Address Validation](#3-rfc-5322822-email-address-validation)
        - [**Regex Pattern:**](#regex-pattern-2)
        - [**Description:**](#description-2)
        - [**Rules:**](#rules-2)
        - [**Examples:**](#examples-2)
      - [4. SIN Validation](#4-sin-validation)
        - [**Description:**](#description-3)
        - [**Rules:**](#rules-3)
        - [Error Messages](#error-messages)

---

## **Authentication Guide**

This API requires authentication via an **API key** provided in the `Authorization` header.

### **Authentication Format**

All requests must include the `Authorization` header with the following format:

```bash
Authorization: Apikey <YOUR_API_KEY>
```

### **Error Responses**

If authentication fails, the API will return one of the following errors:

| Status Code                  | Error Message                                                            | Description                                                                 |
| ---------------------------- | ------------------------------------------------------------------------ | --------------------------------------------------------------------------- |
| **400** Bad Request          | `Invalid Authorization format. Expected: 'Authorization: Apikey <UUID>'` | The `Authorization` header is malformed. Ensure it's in the correct format. |
| **404** Not Found            | `User with API key '<UUID>' not found`                                   | The provided API key does not match any user in the system.                 |
| **422** Unprocessable Entity | `Invalid API key format. Expected a valid UUID.`                         | The API key is not a valid UUID format.                                     |
| **422** Unprocessable Entity | `Missing authentication header`                                          | No `Authorization` header was provided in the request.                      |

:::info

- The API key must be a valid **UUID**.
- If the API key does not belong to a registered user, authentication will fail.
  :::

:::warning
Keep your API key secure and never share it. If you believe your API key has been compromised, contact support immediately for a replacement.
:::

## Error Handling

The API uses standard HTTP status codes to indicate the success or failure of an API request. Common error codes include:

- **400 Bad Request**: The request was invalid or cannot be otherwise served.
- **401 Unauthorized**: Authentication credentials were missing or incorrect.
- **404 Not Found**: The requested resource could not be found.
- **409 Conflict**: The request could not be completed due to a conflict with the current state of the resource.
  - This will occur when attempting to submit a duplicate consumer id record.
- **422 Unprocessable Entity**: The request was recognized but is malformed
  - This will occur if the request body contains invalid or malformed data. See [Data Standards](#b-data-standards)

### Error Responses

All 4XX and 5XX responses will have a content type of "application/json" and have a root key of "error". The value for "error" will either be a string detailing the error, or in the case of a 422 error, a json structure with further error details.

```json
{
  "error": "message"
}
```

or

```json
{
  "error": {
    "key": "value"
  }
}
```

## 1. Submit a consumer credit record

**Endpoint**: `/consumer-credit/{id}`

**Method**: `PUT`

**Description**: Create a new consumer credit record in the system.

**Path Parameters**:

| Parameter | Type   | Description                                      |
| --------- | ------ | ------------------------------------------------ |
| id        | string | Unique identifier for the consumer credit record |

**Response Codes**:

| Code | Description                                                          |
| ---- | -------------------------------------------------------------------- |
| 201  | Created - Consumer credit record successfully created                |
| 409  | Conflict - A consumer credit record with id of `{id}` already exists |

**Request Body**

> **consumer_facts**

| Field                          | Type              | Description                                                                                                                   |
| ------------------------------ | ----------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| first_name                     | string            | First name of the consumer. Must be at least 2 characters.                                                                    |
| last_name                      | string            | Last name of the consumer. Must be at least 2 characters.                                                                     |
| email                          | string            | RFC 5322 and RFC 822 format email address of the consumer                                                                     |
| date_of_birth                  | string            | ISO 8601 date format of the consumer's date of birth                                                                          |
| address                        | string            | CAN/CSA-Z109.1-01 or USPS Publication 28 address format of the consumer                                                       |
| phone_number                   | string            | E.164 international format phone number of the consumer                                                                       |
| SIN/SSN                        | string (optional) | SIN(`NNN-NNN-NNN`) or SSN(`NNN-NN-NNNN`) of the consumer                                                                      |
| institution_names              | array             | List of associated institutions. Each name must be between 2 and 50 characters.                                               |
| consumer_information_indicator | string (optional) | Used to report a special condition of the account. See [B. Consumer Information Indicator](#b-consumer-information-indicator) |

> **credit_facts**

| Field                | Type   | Description                                    |
| -------------------- | ------ | ---------------------------------------------- |
| amount               | float  | Amount requested by borrower, in dollars/cents |
| credit_type          | string | Type of credit (`"PDL"` or `"BNPL"`)           |
| application_datetime | string | ISO 8601 datetime of application               |
| credit_state         | string | State of credit (see values below)             |

> **Credit States**:

- `"application"`
- `"originated"`
- `"declined"`
- `"non-compliant"`
- `"compliant"`

**Example**:

```json
{
  "consumer_facts": {
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "date_of_birth": "2010-10-10",
    "address": "101 1ST. S.W. Calgary AB T2P 2V6",
    "phone_number": "+11234567890",
    "institution_names": ["TD", "RBC"]
  },
  "credit_facts": {
    "amount": 1000.0,
    "credit_type": "PDL",
    "application_datetime": "2024-09-23 21:47:12.023476",
    "credit_state": "applied"
  }
}
```

:::info
The KlearSync Data ETL interface automatically populates new consumer credit records from your system into KlearLink, effectively eliminating the need to submit your records via API.

:::

---

## 2. Update a consumer credit record

**Endpoint**: `/consumer-credit/{id}`

**Method**: `POST`

**Description**: Update the consumer_facts and/or credit_facts of a previously submitted consumer credit record

**Path Parameters**:

| Parameter | Type   | Description                               |
| --------- | ------ | ----------------------------------------- |
| id        | string | ID of the existing consumer credit record |

**Response Codes**:

| Code | Description                                          |
| ---- | ---------------------------------------------------- |
| 200  | OK - Consumer credit record successfully updated     |
| 404  | Not Found - No consumer match found for id of `{id}` |

**Request Body**:

Same schema as Submit endpoint (all fields optional), with additional optional fields in credit_facts:

credit_facts (optional - only present on credit states of 'originated', 'compliant', 'non-compliant')

| Field               | Type   | Description                              |
| ------------------- | ------ | ---------------------------------------- |
| originated_datetime | string | ISO 8601 datetime of origination         |
| payment_due_date    | string | ISO 8601 datetime of payment due date    |
| payment_amount_due  | float  | Amount due for payment, in dollars/cents |

:::info
`originated_datetime`, `payment_due_date`, and `payment_amount_due` are all required if one is provided. Further, these fields can only be set when the credit state is not `application` or `declined`
:::

**Example**:

```json
{
  "consumer_facts": {
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "date_of_birth": "2010-10-10",
    "address": "101 1ST. S.W. Calgary AB T2P 2V6",
    "phone_number": "+11234567890",
    "institution_names": ["TD", "RBC"]
  },
  "credit_facts": {
    "amount": 1000.0,
    "credit_type": "PDL",
    "application_datetime": "2024-09-23 21:47:12.023476",
    "originated_datetime": "2024-09-24 15:43:12.023476",
    "payment_due_date": "2024-09-30 15:43:12.023476",
    "payment_amount_due": 1000.0,
    "credit_state": "originated"
  }
}
```

:::info
The KlearSync Data ETL interface will automatically update your existing consumer credit records in KlearLink, effectively eliminating the need to update them via API.

:::

---

## 3. View a submitted consumer credit record

**Endpoint**: `/consumer-credit/{id}`

**Method**: `GET`

**Description**: View the data associated with a submitted consumer credit record

**Path Parameters**:

| Parameter | Type   | Description                      |
| --------- | ------ | -------------------------------- |
| id        | string | ID of the consumer credit record |

**Response Codes**:

| Code | Description                                          |
| ---- | ---------------------------------------------------- |
| 200  | OK - Request successful                              |
| 404  | Not Found - No consumer match found for id of `{id}` |

**Response Body**:

Includes all fields from consumer_facts and credit_facts, plus:

| Field      | Type    | Description                          |
| ---------- | ------- | ------------------------------------ |
| created_at | string  | ISO 8601 datetime of record creation |
| updated_at | string  | ISO 8601 datetime of last update     |
| processed  | boolean | Whether record has been processed    |

**Example**:

```json
{
  "consumer_facts": {
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "date_of_birth": "2010-10-10",
    "address": "101 1ST. S.W. Calgary AB T2P 2V6",
    "phone_number": "+11234567890",
    "institution_names": ["TD", "RBC"]
  },
  "credit_facts": {
    "amount": 1000.0,
    "credit_type": "PDL",
    "application_datetime": "2024-09-23 21:47:12.023476",
    "originated_datetime": "2024-09-24 15:43:12.023476",
    "payment_due_date": "2024-09-30 15:43:12.023476",
    "payment_amount_due": 1000.0,
    "credit_state": "originated"
  },
  "created_at": "datetime",
  "updated_at": "datetime",
  "processed": true
}
```

---

## 4. View Consumer Match

**Endpoint**: `/consumer-credit/{id}/consumer-match`

**Method**: `GET`

**Description**: View the results of inter-organizational consumer match results on your previously submitted consumer-credit

**Path Parameters**:

| Parameter | Type   | Description                      |
| --------- | ------ | -------------------------------- |
| id        | string | ID of the consumer credit record |

**Response Codes**:

| Code | Description                                          |
| ---- | ---------------------------------------------------- |
| 200  | OK - Consumer match identified                       |
| 404  | Not Found - No consumer match found for id of `{id}` |

**Response Body**:

Includes consumer_facts and credit_facts from original record, plus a `consumer_match` node with the credit_facts, a minimal set of consumer_facts of all matched records, and statistics.

**Example**:

```json
{
  "consumer_facts": {
    "first_name": "John",
    "last_name": "Doe",
    "email": "john.doe@example.com",
    "date_of_birth": "2010-10-10",
    "address": "101 1ST. S.W. Calgary AB T2P 2V6",
    "phone_number": "+11234567890",
    "institution_names": ["TD", "RBC"]
  },
  "credit_facts": {
    "amount": 1000.0,
    "credit_type": "PDL",
    "application_datetime": "2024-09-23 21:47:12.023476",
    "originated_datetime": "2024-09-24 15:43:12.023476",
    "payment_due_date": "2024-09-30 15:43:12.023476",
    "payment_amount_due": 1000.0,
    "credit_state": "originated"
  },
  "created_at": "datetime",
  "updated_at": "datetime",
  "processed": true,
  "consumer_match": [
    {
      "credit_facts": {
        "amount": 1200.0,
        "credit_type": "PDL",
        "application_datetime": "2024-09-23 11:47:12.023476",
        "originated_datetime": "2024-09-24 12:43:12.023476",
        "payment_due_date": "2024-09-30 07:43:12.023476",
        "payment_amount_due": 1200.0,
        "credit_state": "non-compliant"
      },
      "consumer_facts":{
          "consumer_information_indicator": null,
          "institution_names":["TD"]
      }
    }
  ],
  "statistics": {
    "days_since_last_application": 1,
    "days_since_last_origination": 1,
    "average_credit_age": 1.0,
    "number_of_active_loans": 1
  }
}
```

:::info
Here, we see an inter-organizational match indicating that your applicant is non-compliant on a loan originated by another organization.

You do not see what organization the non-compliant loan originated from, nor do you obtain any additional information on the organization, nor do you see any consumer_facts or credit_facts that you do not already have.

The statistics field provides aggregated information about the matched records:
- `days_since_last_application`: Number of days since the most recent application
- `days_since_last_origination`: Number of days since the most recent origination (if any)
- `average_credit_age`: Average age in days of active credit lines
- `number_of_active_loans`: Count of currently outstanding credit lines (originated, compliant, or non-compliant)

:::

:::info
For real-time updates on consumer matches, use the KlearWatch interface.

:::

---

## Security

The KlearLink API is designed with robust security features to ensure the protection of sensitive data and compliance with industry standards.

### Authentication

- The API uses API key-based authentication, requiring both an `X-API-Key` and `X-Username` in the request headers to authenticate requests.

### Monitoring and Logging

- Comprehensive logging is implemented to monitor API usage and detect any unauthorized access attempts. Logs are securely stored and regularly reviewed to ensure system integrity.

### Compliance and Protection

- The API is developed in adherence to the [FAPI 2.0 Security Profile](https://openid.net/specs/fapi-2_0-security-profile-ID2.html), ensuring high standards of financial-grade API security.
- It is also protected against the [OWASP Top Ten](https://owasp.org/www-project-top-ten/) vulnerabilities, providing a secure environment against common security threats.

## Appendix

### A. Definitions

| Term              | Definition                                                                                                                           |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Consumer Credit   | A financial product extended to consumers for personal use, including both Pay Day Loans (PDL) and Buy Now Pay Later (BNPL) products |
| Consumer Facts    | Identifying information about a consumer that can be used for matching and verification purposes                                     |
| Credit Facts      | Details about a specific credit product, including amount, type, and current state                                                   |
| Institution Names | Financial institutions or lenders that have a relationship with the consumer                                                         |
| Consumer State    | The current status of a consumer in relation to their credit products                                                                |
| Credit State      | The current status of a specific credit product                                                                                      |

### B. Consumer Information Indicator

The consumer information indicator of the consumer facts can be the following:

| Value | Description                                |
| ----- | ------------------------------------------ |
| A     | Chapter 7 - Bankruptcy in Canada           |
| B     | Chapter 11 - Proposal in Canada            |
| C     | Chapter 12 (OPD in Canada)                 |
| D     | Chapter 13 - Credit Counselling in Canada  |
| E     | Discharged through Bankruptcy Chapter 7    |
| F     | Discharged Proposal                        |
| G     | Discharged through Bankruptcy Chapter 12   |
| T     | Credit Grantor Cannot Locate Consumer      |
| Z     | Chapter 7 - Bankruptcy in Canada           |
| ZA    | Chapter 7 - Bankruptcy in Canada           |
| ZB    | Chapter 11 - Discharged Proposal in Canada |
| ZC    | Bankruptcy Dismissed                       |
| ZD    | Bankruptcy Withdrawn                       |
| Q     | Removes previously reported Bankruptcy     |

:::info
These are standard definitions from TU reporting guidelines.

:::

### B. Data Standards

| Data Type        | Standard            | Format Example                        | Description                                                                          |
| ---------------- | ------------------- | ------------------------------------- | ------------------------------------------------------------------------------------ |
| Date             | ISO 8601            | `YYYY-MM-DD`                          | International date format. Dates must not be in the future.                          |
| Email            | RFC 5322/822        | `john.doe@example.com`                | RFC 5322 and RFC 822 format email address. Must be between 5 and 254 characters.     |
| DateTime         | ISO 8601            | `YYYY-MM-DD HH:mm:ss.SSSSSS`          | International datetime format with microsecond precision. Must not be in the future. |
| Phone Number     | E.164               | `+1XXXXXXXXXX`                        | International phone number format. Must be between 10 and 15 digits.                 |
| Address (Canada) | CAN/CSA-Z109.1-01   | `101 1ST. S.W. Calgary AB T2P 2V6`    | Canadian postal address format. Must be between 5 and 100 characters.                |
| Address (USA)    | USPS Publication 28 | `1234 MAIN ST NW WASHINGTON DC 20500` | US postal address format. Must be between 5 and 100 characters.                      |
| SIN              | CRA Standard        | `NNN-NNN-NNN`                         | Canadian Social Insurance Number format. Must be exactly 9 digits. Optional.         |
| SSN              | SSA Standard        | `NNN-NN-NNNN`                         | US Social Security Number format. Must be exactly 9 digits. Optional.                |

:::warn
The klearlink API has very strict data validation! All data sent to klearlink must be valid json and the aforementioned key fields MUST adhere to the specified format.
:::

:::info
All monetary values in this API are expressed in the local currency (CAD for Canadian transactions, USD for US transactions) and should be provided as decimal numbers with up to 2 decimal places.
:::

#### 1. E.164 Phone Number Validation

##### **Regex Pattern:**

```regex
^\+?[1-9]\d{1,14}$
```

##### **Description:**

This regex validates phone numbers following the **E.164 international standard**, ensuring they are globally unique and properly formatted.

##### **Rules:**

- The number may start with an optional `+`.
- The country code must be between 1 and 3 digits and cannot start with 0.
- The total length (including country code) must be between 2 and 15 digits.
- Only numeric digits are allowed (no spaces, dashes, or special characters apart from `+`).

##### **Examples:**

✅ Valid:

- `+12025550123`
- `+442071838750`
- `+919876543210`
- `+8613800138000`

❌ Invalid:

- `12025550123` (missing `+` but may be valid in certain systems)
- `+0123456789` (country code cannot start with 0)
- `+9999999999999999` (exceeds 15 digits)
- `+44 207 183 8750` (contains spaces)
- `+1-202-555-0123` (contains dashes)

---

#### 2. CAN/CSA-Z109.1-01 Canadian Address Validation

##### **Regex Pattern:**

```regex
^\d+\s[A-Za-z0-9\s.,'-]+,\s[A-Za-z\s-]+,\s(?:AB|BC|MB|NB|NL|NS|NT|NU|ON|PE|QC|SK|YT),\s[A-Za-z]\d[A-Za-z]\s?\d[A-Za-z]\d(?:,\sCanada)?$
```

##### **Description:**

This regex validates addresses formatted according to the **CAN/CSA-Z109.1-01** standard, which is commonly used in Canada.

##### **Rules:**

- The address must start with a **street number**.
- The **street name** must contain letters, numbers, spaces, and optional characters (`.,'-`).
- The **city name** must contain only letters and spaces.
- The **province/territory code** must be one of the valid two-letter abbreviations: `AB, BC, MB, NB, NL, NS, NT, NU, ON, PE, QC, SK, YT`.
- The **postal code** must follow the format `A1A 1A1` (where `A` is a letter and `1` is a digit) with an optional space.
- The **country name "Canada"** is optional.

##### **Examples:**

✅ Valid:

- `123 Main St, Toronto, ON, M5V 3L9`
- `4567 Elm Ave, Vancouver, BC, V6B 1H2, Canada`
- `77-101 King St W, Hamilton, ON, L8P 1A1`

❌ Invalid:

- `123 Main St Toronto ON M5V 3L9` (missing commas)
- `4567 Elm Ave, Vancouver, BC, 12345` (invalid postal code format)
- `Main St, Toronto, ON, M5V 3L9` (missing street number)
- `123 Fake St, Springfield, XX, M1M 1M1` (invalid province code)

---

#### 3. RFC 5322/822 Email Address Validation

##### **Regex Pattern:**

```regex
^[^\s@]+@[^\s@]+\.[^\s@]+$
```

##### **Description:**

This regex validates email addresses according to the **RFC 5322/822** standard, ensuring that they conform to typical email formatting rules.

##### **Rules:**

- The local part may contain alphanumeric characters, dots, and special characters `(!#$%&'*+/=?^_{|}~-)`.
- The local part may be enclosed in quotes (`"..."`) if special characters are used.
- The domain must contain alphanumeric characters and hyphens, but not start or end with a hyphen.
- The domain must end with a valid top-level domain (2-63 characters in length).

##### **Examples:**

✅ Valid:

- `example@email.com`
- `user.name+tag@example.co.uk`
- `"quoted@text"@example.com`

❌ Invalid:

- `plainaddress` (missing `@` and domain)
- `@missinglocal.com` (missing local part)
- `user@.com` (invalid domain format)
- `user@com` (top-level domain too short)
- `user@-example.com` (hyphen at the start of domain)

---

#### 4. SIN Validation

##### **Description:**

This validator ensures compliance with CRA standards for SIN numbers

##### **Rules:**

- If None → Accept it (SIN is optional).
- If provided:
  - Must be 9 digits long.
  - Must start with 1-9 (no leading zero).
  - Must pass Luhn checksum validation.

##### Error Messages

- "Invalid SIN: XXX. Must be exactly 9 digits."
- "Invalid SIN: XXX. Cannot start with 0."
- "Invalid SIN: XXX. Failed Luhn checksum validation."

---

:::info
Data validation errors are expressed in the following [format](https://docs.rs/serde_valid/latest/serde_valid/#validation-errors-format)
:::
