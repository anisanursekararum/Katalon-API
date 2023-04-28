package schemaAssertion

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable

public class LoginSchemaAssertion {
	
	@Keyword
	def static void loginSchemaAssertion(Object response) {
		String jsonSchema =
				"""
			{
			  "\$id": "https://example.com/person.schema.json",
			  "\$schema": "https://json-schema.org/draft/2020-12/schema",
			  "title": "Person",
			  "type": "object",
			  "properties": {
			    "status": {
			      "type": "string"
			    },
			    "message": {
			      "type": "string"
			    },
			    "user": {
			      "type": "object",
			      "properties": {
				    "id": {
				      "type": "string"
				    },
				    "officeId": {
				      "type": "string"
				    },
				    "companyId": {
				      "type": "string"
				    }
				  }
			    }
			  }
			}
		"""
		boolean successful = WS.validateJsonAgainstSchema(response,jsonSchema)
	}
}
