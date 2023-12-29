import os
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.common.keys import Keys
import time

driver = webdriver.Chrome()

# Get the email from the environment variable
email = os.getenv('TokenGenerationBot-Email')
# Make sure the email was set
if not email:
    raise ValueError('The TokenGenerationBot-Email environment variable is not set.')

password = os.getenv('TokenGenerationBot-Password')
if not password:
    raise ValueError('The TokenGenerationBot-Password environment variable is not set.')

# Function to create a token and return its ID
def create_token(token_number):
    driver.get("https://www.start.gg/admin/user/18120ae0/developer")

    login_field = driver.find_element(By.XPATH, "//input[@placeholder='john.smith@gmail.com']")
    login_field.send_keys(email)

    password_field = driver.find_element(By.XPATH, "//input[@placeholder='Password']")
    password_field.send_keys(password)

    wait = WebDriverWait(driver, 10)  # Wait for up to 10 seconds

    try:
        accept_button = wait.until(EC.element_to_be_clickable((By.ID, 'onetrust-accept-btn-handler')))
        accept_button.click()
    except Exception as e:
        # If the "Accept" button is not present, just proceed
        print("No cookie consent button found or it's not clickable.")

    # Find the button with the validationkey "LOGIN_userlogin"
    login_button = driver.find_element(By.XPATH, "//button[@validationkey='LOGIN_userlogin']")
    login_button.click()


    # Find and click the "Create new token" button
    create_button = wait.until(EC.element_to_be_clickable((By.CSS_SELECTOR, '.MuiButton-contained.tss-1cz5t4l-containedPrimary')))
    create_button.click()

    # Wait for any necessary loading
    time.sleep(2)

    # Fill in the token name
    token_name_field = driver.find_element(By.CSS_SELECTOR, '.MuiOutlinedInput-input')
    token_name_field.send_keys(f"pidgtm-compile-{token_number}")

    # Submit the form / click the submit button
    token_name_field.send_keys(Keys.RETURN)

    time.sleep(2)

    # Find all <strong> elements on the page
    strong_elements = driver.find_elements(By.TAG_NAME, "strong")

    # Check if there are any <strong> elements on the page
    if strong_elements:
        # Get the last <strong> element in the list (which is the last one on the page)
        last_strong_element = strong_elements[-1]
        return last_strong_element.text
    else:
        raise ValueError('Error generating token')

# Open tokens.txt file to write tokens
with open('tokens.txt', 'w') as file:
    for i in range(1, 5): # StartGG has a limit of 5 tokens per user
        token_id = create_token(i)
        file.write(token_id + '\n')

# Close the browser
driver.quit()
