import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.github.javafaker.Faker as Faker

def faker = new Faker()

GlobalVariable.name = faker.name().fullName()

GlobalVariable.email = faker.internet().emailAddress()

GlobalVariable.password = faker.internet().password()

System.out.println(GlobalVariable.name)

System.out.println(GlobalVariable.email)

System.out.println(GlobalVariable.password)

response = WS.sendRequest(findTestObject('Auth/registration'))

WS.verifyResponseStatusCode(response, 201)

WS.verifyElementPropertyValue(response, 'status', 'success')

WS.verifyElementPropertyValue(response, 'message', 'Toko berhasil didaftarkan')

WS.verifyElementPropertyValue(response, 'data.name', GlobalVariable.name)

WS.verifyElementPropertyValue(response, 'data.email', GlobalVariable.email)

